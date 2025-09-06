use pbsfm_rss_feed::Args;
use tempdir::TempDir;
use std::fs;
use rss_gen::parse_rss;

mod mock_airnet;

#[test]
fn test_cli_e2e() -> Result<(), Box<dyn std::error::Error>> {
    let server = mock_airnet::start_mock_airnet_server()?;
    let tmp_dir = TempDir::new("output")?;
    let args = Args {
        airnet_url: server.base_url(),
        programs: vec!("black-wax".to_string()),
        output_dir: tmp_dir.path().to_path_buf(),
    };
    pbsfm_rss_feed::run_app(args)?;

    let output_file = tmp_dir.path().join("pbsfm/black-wax/rss.xml");
    let contents = fs::read_to_string(output_file)?;
    let actual_rss = parse_rss(contents.as_str(), None)?;

    assert_eq!(actual_rss.title, mock_airnet::expected::single_program().name);
    assert_eq!(actual_rss.item_count(), mock_airnet::expected::episodes().len());
    Ok(())
}