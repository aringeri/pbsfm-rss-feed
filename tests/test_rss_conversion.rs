mod mock_airnet;

#[test]
fn test_convert_to_rss() -> Result<(), Box<dyn std::error::Error>> {
    let program = mock_airnet::expected::single_program();
    let episodes = mock_airnet::expected::episodes();

    let rss_feed = pbsfm_rss_feed::convert_to_rss(program.clone(), episodes.clone())?;

    assert_eq!(mock_airnet::expected::rss_feed(), rss_feed);
    Ok(())
}

#[test]
fn test_fetch_and_convert() -> Result<(), Box<dyn std::error::Error>> {
    let server = mock_airnet::start_mock_airnet_server()?;

    let rss_feed = pbsfm_rss_feed::generate_rss_feed(
        server.base_url(),
        "3pbs",
        "black-wax"
    )?;

    let program = mock_airnet::expected::single_program();
    let episodes = mock_airnet::expected::episodes();

    assert_eq!(rss_feed.title, program.name);
    assert_eq!(rss_feed.items.len(), episodes.len());

    Ok(())
}