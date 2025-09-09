use crate::airnet::types::{Episode, PlaylistItem, ProgramDetails};
use clap::Parser;
use quick_xml::writer::Writer;
use rss_gen::macro_write_element;
use rss_gen::{RssData, RssItem, RssVersion};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

pub mod airnet;
pub mod rss_macros;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "https://airnet.org.au")]
    pub airnet_url: String,

    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    pub programs: Vec<String>,

    #[arg(short, long, default_value = "docs/feeds/")]
    pub output_dir: PathBuf,
}

const PBSFM_STATION: &str = "3pbs";

pub fn run_app(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    for program in args.programs {
        let rss_feed = generate_rss_feed(&args.airnet_url, &*program)?;
        let station_dir = args.output_dir.join("pbsfm/").join(program);
        std::fs::create_dir_all(&station_dir)?;

        let out_file = File::create(station_dir.join("rss.xml"))?;
        let writer = Writer::new(BufWriter::new(out_file));
        let r: Result<_, std::io::Error> =
            macro_generate_rss_custom!(writer, rss_feed).map(|_writer| ());
        r?;
    }
    Ok(())
}

pub fn generate_rss_feed(
    airnet_url: &String,
    program_name: &str,
) -> Result<RssData, Box<dyn std::error::Error>> {
    let client = airnet::AirnetClient::new(airnet_url.clone());
    let program = client.program(PBSFM_STATION, program_name)?;
    println!("Fetched program: {}", program.name);
    let episodes = client.episodes(PBSFM_STATION, program_name)?;
    println!("Fetched episodes: {}", episodes.len());

    let mut episodes_and_playlists: Vec<(Episode, Vec<PlaylistItem>)> = Vec::new();
    for episode in episodes{
        println!("Fetching episode playlist {}", episode.start);
        let playlist: Vec<PlaylistItem> = client.episode_playlist(
            PBSFM_STATION, 
            program_name, 
            format!("{}", episode.start.format("%Y-%m-%d+%H:%M:%S")).as_str()
        )?;
        episodes_and_playlists.push((episode, playlist));
    }

    convert_to_rss(program, episodes_and_playlists)
}

fn convert_to_rss(
    program: ProgramDetails,
    episodes: Vec<(Episode, Vec<PlaylistItem>)>,
) -> Result<RssData, Box<dyn std::error::Error>> {
    let mut rss_data = RssData::new(Some(RssVersion::RSS2_0))
        .link(format!("https://www.pbsfm.org.au/program/{}", program.slug))
        .title(program.name)
        .description(program.description)
        .author(&program.broadcasters)
        .image_url(program.profile_image_url)
        .language("en");

    for (episode, playlist) in episodes {
        println!("Writing episode: {:?}, {}", episode.title, episode.start);
        let title = &episode
            .title
            .unwrap_or(format!("Untitled - {}", episode.start.format("%Y-%m-%d")));
        let episode_link = format!(
            "https://www.pbsfm.org.au/program/{}/{}/{}",
            program.slug,
            episode.start.format("%Y-%m-%d"),
            episode.start.format("%H-%M-%S")
        );

        rss_data.add_item(
            RssItem::new()
                .title(title)
                .link(&episode_link)
                .guid(&episode_link)
                .author(&program.broadcasters)
                .description(make_description(playlist))
                .enclosure(format!(
                    "https://airnet.org.au/omnystudio/3pbs/{}/{}/aac_mid.m4a",
                    program.slug,
                    episode.start.format("%Y-%m-%d+%H:%M:%S")
                ))
                .pub_date(episode.start.date().to_string()),
        );
    }
    Ok(rss_data)
}

fn make_description(playlist: Vec<PlaylistItem>) -> String {
    let mut description = String::from("<h3>Tracklist</h3>");
    for track in playlist {
        description.push_str(format!("<br><b>{}</b>", track.artist).as_str());
        if track.title.is_some() {
            description.push_str(" ");
            description.push_str(track.title.unwrap().as_str());
        }
        if track.release.is_some() {
            description.push_str(format!(" (Album: {})", track.release.unwrap()).as_str());
        }
    }
    description
}
