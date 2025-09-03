use std::iter::zip;

mod mock_airnet;

#[test]
fn test_convert_to_rss() -> Result<(), Box<dyn std::error::Error>> {
    let program = mock_airnet::expected::single_program();
    let episodes = mock_airnet::expected::episodes();

    let rss_feed = pbsfm_rss_feed::convert_to_rss(program.clone(), episodes.clone())?;

    assert_eq!(format!("https://www.pbsfm.org.au/program/{}", program.slug), rss_feed.link);
    assert_eq!(program.name, rss_feed.title);
    assert_eq!(program.description, rss_feed.description);
    assert_eq!(program.broadcasters, rss_feed.author);
    assert_eq!(program.profile_image_url, rss_feed.image_link);
    assert_eq!("en", rss_feed.language);

    assert_eq!(episodes.len(), rss_feed.item_count());

    
    for (episode, item) in zip(episodes, rss_feed.items) {
        // assert_eq!(format!("{}", episode.title.map_or("", |a| {a.as_str()})), "")
        assert_eq!(episode.title.unwrap_or("".to_string()), item.title);
        assert_eq!(program.broadcasters, item.author);
        assert_eq!(
            format!("https://airnet.org.au/omnystudio/3pbs/{}/{}/aac_mid.m4a", program.slug, episode.start.to_string()), 
            item.enclosure.unwrap()
        )
    }

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