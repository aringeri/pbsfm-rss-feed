mod mock_airnet;

#[test]
fn test_fetch_and_convert() -> Result<(), Box<dyn std::error::Error>> {
    let server = mock_airnet::start_mock_airnet_server()?;

    let rss_feed = pbsfm_rss_feed::generate_rss_feed(
        &server.base_url(),
        "black-wax",
    )?;

    assert_eq!(mock_airnet::expected::rss_feed(), rss_feed);
    Ok(())
}