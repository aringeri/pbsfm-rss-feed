use quick_xml;
use crate::airnet::types::{Episode, ProgramDetails};
use clap::Parser;
use quick_xml::writer::Writer;
use rss_gen::macro_write_element;
use rss_gen::{RssData, RssItem, RssVersion};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use quick_xml::events::{BytesDecl, Event};
use regex::Regex;
use crate::rss::{CategoryBuilder, ChannelBuilder, Enclosure, ImageBuilder, Item, ItemBuilder, ItemGuidBuilder, Rss};

pub mod airnet;
pub mod rss_macros;
pub mod rss;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "https://airnet.org.au")]
    pub airnet_url: String,

    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    pub programs: Vec<String>,

    #[arg(short, long, default_value = "docs/feeds/")]
    pub output_dir: PathBuf,

    #[arg(short, long, default_value_t = false)]
    pub use_custom_rss_serialization: bool,
}

const PBSFM_STATION: &str = "3pbs";

pub fn run_app(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    for program in args.programs {

        let station_dir = args.output_dir.join("pbsfm/").join(&program);
        std::fs::create_dir_all(&station_dir)?;

        let out_file = File::create(station_dir.join("rss.xml"))?;
        let mut writer = Writer::new_with_indent(BufWriter::new(out_file), b' ', 2);

        if args.use_custom_rss_serialization {
            let rss_feed = generate_rss_feed(&args.airnet_url, &*program, convert_to_rss_v2)?;

            writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;
            writer.write_serializable("rss", &rss_feed)?;
        } else {
            let rss_feed = generate_rss_feed(&args.airnet_url, &*program, convert_to_rss)?;

            let r: Result<_, std::io::Error> =
                macro_generate_rss_custom!(writer, rss_feed).map(|_writer| ());
            r?;
        }
    }
    Ok(())
}
pub fn generate_rss_feed<RSS>(
    airnet_url: &String,
    program_name: &str,
    generate_rss_f: impl Fn(ProgramDetails, Vec<Episode>) -> Result<RSS, Box<dyn std::error::Error>>,
) -> Result<RSS, Box<dyn std::error::Error>> {
    let client = airnet::AirnetClient::new(airnet_url.clone());
    let program = client.program(PBSFM_STATION, program_name)?;
    println!("Fetched program: {}", program.name);
    let episodes = client.episodes(PBSFM_STATION, program_name)?;
    println!("Fetched episodes: {}", episodes.len());

    generate_rss_f(program, episodes)
}

pub fn convert_to_rss_v2(
    program: ProgramDetails,
    episodes: Vec<Episode>,
) -> Result<rss::Rss, Box<dyn std::error::Error>> {
    let program_link = format!("https://www.pbsfm.org.au/program/{}", program.slug);

    let empty_str = "".to_string();
    let rss_data = Rss::new(
        ChannelBuilder::new(
            &program.name,
            &program_link,
            &program.description
        )
            .category(CategoryBuilder::new(
                program.grid_description.as_ref().unwrap_or(&empty_str)
            ).build())
            .image(
                ImageBuilder::new(
                    rm_query_params(&program.profile_image_url)?,
                    &program.name,
                    &program_link,
                ).build()
            )
            .language("en")
            .item(convert_to_items_v2(&program, episodes))
            .build()
    );
    Ok(rss_data)
}
fn convert_to_items_v2(
    program: &ProgramDetails,
    episodes: Vec<Episode>
) -> Vec<Item> {
    episodes.iter().map( |episode| {
        println!("Writing episode: {:?}, {}", episode.title, episode.start);

        let default_title = format!("Untitled - {}", episode.start.format("%Y-%m-%d"));

        let title = episode
            .title
            .as_ref()
            .unwrap_or(&default_title);
        let episode_link = format!(
            "https://www.pbsfm.org.au/program/{}/{}/{}",
            program.slug,
            episode.start.format("%Y-%m-%d"),
            episode.start.format("%H-%M-%S")
        );

        let empty_str = "".to_string();
        let description = episode.description.as_ref().unwrap_or(&empty_str);

        ItemBuilder::with_title(title)
            .link(&episode_link)
            .guid(ItemGuidBuilder::new(&episode_link).build())
            .author(&program.broadcasters)
            .description(description)
            .enclosure(
                Enclosure::new(
                    format!(
                        "https://airnet.org.au/omnystudio/3pbs/{}/{}/aac_mid.m4a",
                        program.slug,
                        episode.start.format("%Y-%m-%d+%H:%M:%S")
                    ),
                    999, //TODO accessible?
                    "audio/mp4"
                )
            )
            .pub_date(episode.start.date().to_string())
            .build()
    }).collect()
}

pub fn convert_to_rss(
    program: ProgramDetails,
    episodes: Vec<Episode>,
) -> Result<RssData, Box<dyn std::error::Error>> {
    let mut rss_data = RssData::new(Some(RssVersion::RSS2_0))
        .link(format!("https://www.pbsfm.org.au/program/{}", program.slug))
        .title(program.name)
        .description(program.description)
        .category(program.grid_description.unwrap_or_default())
        .author(&program.broadcasters)
        .image_url(rm_query_params(&program.profile_image_url)?)
        .language("en");

    for episode in episodes {
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
                .description(episode.description.unwrap_or_default())
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

fn rm_query_params(url: &str) -> Result<String,Box<dyn std::error::Error>> {
    let reg = Regex::new(r"\?.*$")?;
    Ok(reg.replace(url, "").to_string())
}