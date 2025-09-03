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
        .link(format!("https://www.pbsfm.org.au/program/{}", program.slug))
        .title(program.name)
        .description(program.description)
        .author(program.broadcasters.clone())
        .image_link(program.profile_image_url)
        .language("en");

    for episode in episodes {
        println!("Writing episode: {:?}, {}", episode.title, episode.start);
        // let name = format!("{:?}-{}", episode.title.unwrap_or_default(), episode.start);
        rss_data.add_item(
            RssItem::new()
                .title(episode.title.unwrap_or_default())
                .author(program.broadcasters.clone())
                .description(episode.description.unwrap_or(String::from("")))
                .enclosure(format!(
                    "https://airnet.org.au/omnystudio/3pbs/{}/{}/aac_mid.m4a",
                    program.slug,
                    episode.start.format("%Y-%m-%d+%H:%M:%S")
                ))
                .pub_date(episode.start.to_string()),
        );
    }

    println!("RSS feed has items: {}", rss_data.items.len());
    Ok(rss_data)
}
