pub mod airnet;

use clap::Parser;
use pbsfm_rss_feed::Args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args_str = argfile::expand_args(
        argfile::parse_fromfile,
        argfile::PREFIX
    )?;
    let args = Args::parse_from(args_str);
    pbsfm_rss_feed::run_app(args)
}
