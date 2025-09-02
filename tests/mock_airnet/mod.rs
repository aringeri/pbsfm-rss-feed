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
        when.method("GET").path("/rest/stations/3pbs/programs/black-wax");
        then.status(200).body(single_program);
    });

    let episodes = fs::read_to_string("tests/mock_airnet/responses/episodes.json")?;
    server.mock(|when, then| {
        when.method("GET").path("/rest/stations/3pbs/programs/black-wax/episodes");
        then.status(200).body(episodes);
    });

    Ok(server)
}

pub mod expected {
    use pbsfm_rss_feed::airnet::types::{Episode, ProgramDescription, ProgramDetails};
    use chrono::{NaiveDate};

    pub fn all_programs() -> Vec<ProgramDescription> {
        vec!(ProgramDescription{
                slug: None,
                name: String::from("Tomorrow Land"),
                broadcasters: String::from(""),
                grid_description: None,
                archived: true,
                program_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/"),
            },
            ProgramDescription{
                slug: Some(String::from("black-wax")),
                name: String::from("Black Wax"),
                broadcasters: String::from("Adam Rudegeair"),
                grid_description: Some(String::from("Groovin' jazz")),
                archived: false,
                program_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/black-wax"),
            }
        )
    }

    pub fn single_program() -> ProgramDetails {
        ProgramDetails{
            url: None,
            name: String::from("Black Wax"),
            broadcasters: String::from("Adam Rudegeair"),
            description: String::from("Jazz and funk with jazz influence"),
            grid_description: Some(String::from("Groovin' jazz")),
            slug: String::from("black-wax"),
            banner_image_url: String::from("https://banner.jpg"),
            banner_image_small: String::from("https://banner-small.jpg"),
            profile_image_url: String::from("https://profile-img.jpg"),
            profile_image_small: String::from("https://profile-img-small.jpg"),
            episodes_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/black-wax/episodes"),
        }
    }

    pub fn episodes() -> Vec<Episode> {
        vec!(
            Episode{
                url: None,
                start: NaiveDate::from_ymd_opt(2025, 6, 16).unwrap()
                    .and_hms_opt(11, 0, 0).unwrap(),
                end: NaiveDate::from_ymd_opt(2025, 6, 16).unwrap()
                    .and_hms_opt(13, 0, 0).unwrap(),
                duration: 7200,
                title: String::from("Interview with Vince Jones and Jacob Collier!"),
                description: None,
                image_url: None,
                episode_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/black-wax/episodes/2025-06-16+11%3A00%3A00"),
            },
            Episode{
                url: Some(String::from("http://url")),
                start: NaiveDate::from_ymd_opt(2025, 8, 25).unwrap()
                    .and_hms_opt(11, 0, 0).unwrap(),
                end: NaiveDate::from_ymd_opt(2025, 8, 25).unwrap()
                    .and_hms_opt(13, 0, 0).unwrap(),
                duration: 7200,
                title: String::from("Paul Grabowsky Feature"),
                description: Some(String::from("some description")),
                image_url: Some(String::from("http://img-url")),
                episode_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/black-wax/episodes/2025-08-25+11%3A00%3A00"),
            },
        )
    }
}