use crate::airnet::types::{Episode, ProgramDetails};
use clap::Parser;
use quick_xml::writer::Writer;
use rss_gen::macro_write_element;
use rss_gen::{RssData, RssItem, RssVersion};
use std::fs::File;
use std::io::BufWriter;

pub mod airnet;
pub mod rss_macros;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "https://airnet.org.au")]
    pub airnet_url: String,

    #[arg(short, long, default_value = "3pbs")]
    pub station: String,

    #[arg(short, long)]
    pub program: String,

    #[arg(short, long, default_value = "feed.xml")]
    pub output_feed: String,
}
pub fn run_app(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let rss_feed = generate_rss_feed(args.airnet_url, &*args.station, &*args.program)?;

    let writer = Writer::new(BufWriter::new(File::create(args.output_feed)?));
    macro_generate_rss_custom!(writer, rss_feed).map(|_writer| ())
}

pub fn generate_rss_feed(
    airnet_url: String,
    station_name: &str,
    program_name: &str,
) -> Result<RssData, Box<dyn std::error::Error>> {
    let client = airnet::AirnetClient::new(airnet_url);
    let program = client.program(station_name, program_name)?;
    println!("Fetched program: {}", program.name);
    let episodes = client.episodes(station_name, program_name)?;
    println!("Fetched episodes: {}", episodes.len());

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
        println!("Writing episode: {:?}, {}", episode.title, episode.start);
        let name = format!("{:?}-{}", episode.title, episode.start);
        rss_data.add_item(
            RssItem::new()
                .title(name)
                .description(episode.description.unwrap_or(String::from("")))
                .pub_date(episode.start.to_string()),
        );
    }

    println!("RSS feed has items: {}", rss_data.items.len());
    Ok(rss_data)
}
