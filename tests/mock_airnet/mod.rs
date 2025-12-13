use httpmock::prelude::*;
use std::fs;

pub fn start_mock_airnet_server() -> Result<MockServer, std::io::Error> {
    let all_programs = fs::read_to_string("tests/mock_airnet/responses/all-programs.json")?;
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method("GET").path("/rest/stations/3pbs/programs");
        then.status(200).body(all_programs);
    });

    let single_program = fs::read_to_string("tests/mock_airnet/responses/program.json")?;
    server.mock(|when, then| {
        when.method("GET")
            .path("/rest/stations/3pbs/programs/black-wax");
        then.status(200).body(single_program);
    });

    let episodes = fs::read_to_string("tests/mock_airnet/responses/episodes.json")?;
    server.mock(|when, then| {
        when.method("GET")
            .path("/rest/stations/3pbs/programs/black-wax/episodes");
        then.status(200).body(episodes);
    });

    server.mock(|when, then| {
        when.method("GET").any_request();
        then.status(404).body("Not Found");
    });

    Ok(server)
}

pub mod expected {
    use chrono::NaiveDate;
    use pbsfm_rss_feed::airnet::types::{Episode, ProgramDescription, ProgramDetails};
    use pbsfm_rss_feed::rss;
    use pbsfm_rss_feed::rss::{Enclosure, ImageBuilder, Item, ItemBuilder, ItemGuidBuilder};
    use rss_gen::{RssData, RssItem, RssVersion};

    #[allow(dead_code)]
    pub fn all_programs() -> Vec<ProgramDescription> {
        vec![
            ProgramDescription {
                slug: None,
                name: String::from("Tomorrow Land"),
                broadcasters: String::from(""),
                grid_description: None,
                archived: true,
                program_rest_url: String::from(
                    "https://airnet.org.au/rest/stations/3pbs/programs/",
                ),
            },
            ProgramDescription {
                slug: Some(String::from("black-wax")),
                name: String::from("Black Wax"),
                broadcasters: String::from("Adam Rudegeair"),
                grid_description: Some(String::from("Groovin' jazz")),
                archived: false,
                program_rest_url: String::from(
                    "https://airnet.org.au/rest/stations/3pbs/programs/black-wax",
                ),
            },
        ]
    }

    pub fn single_program() -> ProgramDetails {
        ProgramDetails {
            url: None,
            name: String::from("Black Wax"),
            broadcasters: String::from("Adam Rudegeair"),
            description: String::from("Jazz and funk with jazz influence"),
            grid_description: Some(String::from("Groovin' jazz")),
            slug: String::from("black-wax"),
            banner_image_url: String::from("https://banner.jpg?cacbeb=80406601"),
            banner_image_small: String::from("https://banner-small.jpg?cacbeb=80406601"),
            profile_image_url: String::from("https://profile-img.jpg?cacbeb=80406601"),
            profile_image_small: String::from("https://profile-img-small.jpg?cacbeb=80406601"),
            episodes_rest_url: String::from(
                "https://airnet.org.au/rest/stations/3pbs/programs/black-wax/episodes",
            ),
        }
    }

    pub fn episodes() -> Vec<Episode> {
        vec![
            Episode {
                url: None,
                start: NaiveDate::from_ymd_opt(2025, 6, 16)
                    .unwrap()
                    .and_hms_opt(11, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2025, 6, 16)
                    .unwrap()
                    .and_hms_opt(13, 0, 0)
                    .unwrap(),
                duration: 7200,
                title: Some(String::from(
                    "Interview with Vince Jones and Jacob Collier!",
                )),
                description: None,
                image_url: None,
                episode_rest_url: String::from(
                    "https://airnet.org.au/rest/stations/3pbs/programs/black-wax/episodes/2025-06-16+11%3A00%3A00",
                ),
            },
            Episode {
                url: Some(String::from("http://url")),
                start: NaiveDate::from_ymd_opt(2025, 8, 25)
                    .unwrap()
                    .and_hms_opt(11, 0, 0)
                    .unwrap(),
                end: NaiveDate::from_ymd_opt(2025, 8, 25)
                    .unwrap()
                    .and_hms_opt(13, 0, 0)
                    .unwrap(),
                duration: 7200,
                title: None,
                description: Some(String::from("some description")),
                image_url: Some(String::from("http://img-url")),
                episode_rest_url: String::from(
                    "https://airnet.org.au/rest/stations/3pbs/programs/black-wax/episodes/2025-08-25+11%3A00%3A00",
                ),
            },
        ]
    }

    #[allow(dead_code)]
    pub fn rss_feed() -> RssData {
        let program = single_program();

        let mut rss_feed = RssData::new(Some(RssVersion::RSS2_0))
            .link("https://www.pbsfm.org.au/program/black-wax")
            .title(&program.name)
            .description(&program.description)
            .category(
                program
                    .grid_description
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or(""),
            )
            .author(&program.broadcasters)
            .image_url("https://profile-img.jpg")
            .language("en");

        for item in rss_items(&program) {
            rss_feed.add_item(item);
        }
        rss_feed
    }

    #[allow(dead_code)]
    pub fn rss_data() -> rss::Rss {
        let program = single_program();
        let program_link = "https://www.pbsfm.org.au/program/black-wax";

        let category = program
            .grid_description
            .as_ref()
            .map(String::to_string)
            .unwrap_or_default();

        let rss_feed = rss::Rss::new(
            rss::ChannelBuilder::new(&program.name, program_link, &program.description)
                .category(rss::CategoryBuilder::new(category).build())
                .image(
                    ImageBuilder::new("https://profile-img.jpg", &program.name, program_link)
                        .build(),
                )
                .language("en".to_owned())
                .item(rss_items_v2(&program))
                .build(),
        );
        rss_feed
    }

    #[allow(dead_code)]
    pub fn rss_items(program: &ProgramDetails) -> Vec<RssItem> {
        vec!(
            RssItem::new()
                .title(String::from("Interview with Vince Jones and Jacob Collier!"))
                .link("https://www.pbsfm.org.au/program/black-wax/2025-06-16/11-00-00")
                .guid("https://www.pbsfm.org.au/program/black-wax/2025-06-16/11-00-00")
                .author(&program.broadcasters)
                .description("")
                .enclosure("https://airnet.org.au/omnystudio/3pbs/black-wax/2025-06-16+11:00:00/aac_mid.m4a")
                .pub_date("2025-06-16"),
            RssItem::new()
                .title(String::from("Untitled - 2025-08-25"))
                .link("https://www.pbsfm.org.au/program/black-wax/2025-08-25/11-00-00")
                .guid("https://www.pbsfm.org.au/program/black-wax/2025-08-25/11-00-00")
                .author(&program.broadcasters)
                .description("some description")
                .enclosure("https://airnet.org.au/omnystudio/3pbs/black-wax/2025-08-25+11:00:00/aac_mid.m4a")
                .pub_date("2025-08-25"),
        )
    }

    #[allow(dead_code)]
    pub fn rss_items_v2(program: &ProgramDetails) -> Vec<Item> {
        vec!(
            ItemBuilder::with_title("Interview with Vince Jones and Jacob Collier!")
                .link("https://www.pbsfm.org.au/program/black-wax/2025-06-16/11-00-00".to_owned())
                .guid(ItemGuidBuilder::new("https://www.pbsfm.org.au/program/black-wax/2025-06-16/11-00-00").build())
                .author(program.broadcasters.clone())
                .enclosure(Enclosure::new(
                    "https://airnet.org.au/omnystudio/3pbs/black-wax/2025-06-16+11:00:00/aac_mid.m4a",
                    None,
                    "audio/mp4"
                ))
                .pub_date("2025-06-16".to_owned())
                .build(),
            ItemBuilder::with_title("Untitled - 2025-08-25")
                .link("https://www.pbsfm.org.au/program/black-wax/2025-08-25/11-00-00".to_owned())
                .guid(ItemGuidBuilder::new("https://www.pbsfm.org.au/program/black-wax/2025-08-25/11-00-00").build())
                .author(program.broadcasters.clone())
                .description("some description".to_string())
                .enclosure(Enclosure::new(
                    "https://airnet.org.au/omnystudio/3pbs/black-wax/2025-08-25+11:00:00/aac_mid.m4a",
                    None,
                    "audio/mp4"
                ))
                .pub_date("2025-08-25".to_owned())
                .build()
        )
    }
}
