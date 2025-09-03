pub mod airnet;

use clap::Parser;
use pbsfm_rss_feed::Args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    pbsfm_rss_feed::run_app(args)
}
