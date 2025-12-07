use pbsfm_rss_feed::Args;
use tempdir::TempDir;
use std::fs;

mod mock_airnet;

#[test]
fn test_cli_e2e() -> Result<(), Box<dyn std::error::Error>> {
    let server = mock_airnet::start_mock_airnet_server()?;
    let tmp_dir = TempDir::new("output")?;
    let args = Args {
        airnet_url: server.base_url(),
        programs: vec!("black-wax".to_string()),
        output_dir: tmp_dir.path().to_path_buf(),
        use_custom_rss_serialization: false,
    };
    pbsfm_rss_feed::run_app(args)?;

    let output_file = tmp_dir.path().join("pbsfm/black-wax/rss.xml");
    let contents = fs::read_to_string(output_file)?;
    let expected_contents = fs::read_to_string("tests/expected-black-wax.rss")?;
    assert_eq!(contents, expected_contents);
    Ok(())
}

#[test]
fn test_cli_e2e_with_custom_rss_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let server = mock_airnet::start_mock_airnet_server()?;
    let tmp_dir = TempDir::new("output")?;
    let args = Args {
        airnet_url: server.base_url(),
        programs: vec!("black-wax".to_string()),
        output_dir: tmp_dir.path().to_path_buf(),
        use_custom_rss_serialization: true,
    };
    pbsfm_rss_feed::run_app(args)?;

    let output_file = tmp_dir.path().join("pbsfm/black-wax/rss.xml");
    let contents = fs::read_to_string(output_file)?;
    let expected_contents = fs::read_to_string("tests/expected-black-wax-v2.rss")?;
    assert_eq!(contents, expected_contents);
    Ok(())
}