use crate::airnet::types::{Episode, ProgramDescription};
use rss_gen::{RssData, RssItem, RssVersion};

pub mod airnet;

fn convert_to_rss(
    program: ProgramDescription,
    episodes: Vec<Episode>,
) -> Result<RssData, Box<dyn std::error::Error>> {
    let mut rss_data = RssData::new(Some(RssVersion::RSS2_0))
        .title(program.name)
        .description(program.grid_description.unwrap_or(String::from("")));

    for episode in episodes {
        rss_data.add_item(RssItem::new()
            .title(episode.title)
            .description(episode.description.unwrap_or(String::from("")))
            .pub_date(episode.start.to_string())
        );
    }

    Ok(rss_data)
}

mod test {
    use chrono::NaiveDate;
    use rss_gen::{macro_generate_rss, macro_write_element};
    use crate::airnet::types::{Episode, ProgramDescription};
    use quick_xml::writer::Writer;
    use std::io::{Error};
    use std::fs::File;
    use std::io::BufWriter;

    #[test]
    fn test_convert_to_rss() -> Result<(), Box<dyn std::error::Error>> {
        let program = ProgramDescription{
            slug: Some(String::from("black-wax")),
            name: String::from("Black Wax"),
            broadcasters: String::from("Adam Rudegeair"),
            grid_description: Some(String::from("Groovin' jazz")),
            archived: false,
            program_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/black-wax"),
        };
        let episodes = vec!(Episode{
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
        });

        let rss_feed = crate::convert_to_rss(program, episodes)?;
        let writer = Writer::new(BufWriter::new(File::create("output.xml")?));
        let result: Result<Writer<BufWriter<File>>, Error> = macro_generate_rss!(writer, rss_feed);
        assert!(result.is_ok());
        Ok(())
    }
}
