use crate::airnet::types::{Episode, ProgramDetails};
use rss_gen::{RssData, RssItem, RssVersion};

pub mod airnet;

pub fn generate_rss_feed(
    airnet_url: String,
    station_name: &str,
    program_name: &str,
) -> Result<RssData, Box<dyn std::error::Error>> {
    let client = airnet::AirnetClient::new(airnet_url);
    let program = client.program(station_name, program_name)?;
    let episodes = client.episodes(station_name, program_name)?;

    convert_to_rss(program, episodes)
}

pub fn convert_to_rss(
    program: ProgramDetails,
    episodes: Vec<Episode>,
) -> Result<RssData, Box<dyn std::error::Error>> {
    let mut rss_data = RssData::new(Some(RssVersion::RSS2_0))
        .title(program.name)
        .description(program.grid_description.unwrap_or(String::from("")));

    for episode in episodes {
        rss_data.add_item(
            RssItem::new()
                .title(episode.title)
                .description(episode.description.unwrap_or(String::from("")))
                .pub_date(episode.start.to_string()),
        );
    }

    Ok(rss_data)
}
