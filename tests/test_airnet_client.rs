use pbsfm_rss_feed::airnet;

mod mock_airnet;

#[test]
fn test_get_all_programs() -> Result<(), Box<dyn std::error::Error>> {
    let server = mock_airnet::start_mock_airnet_server()?;

    let client = airnet::AirnetClient::new(server.base_url());
    let programs = client.all_programs("3pbs")?;

    assert_eq!(
        programs,
        mock_airnet::expected::all_programs()
    );
    Ok(())
}

#[test]
fn test_get_program() -> Result<(), Box<dyn std::error::Error>> {
    let server = mock_airnet::start_mock_airnet_server()?;

    let client = airnet::AirnetClient::new(server.base_url());
    let program = client.program("3pbs", "black-wax")?;

    assert_eq!(
        program,
        mock_airnet::expected::single_program()
    );
    Ok(())
}

#[test]
fn test_get_episode_list() -> Result<(), Box<dyn std::error::Error>> {
    let server = mock_airnet::start_mock_airnet_server()?;

    let client = airnet::AirnetClient::new(server.base_url());
    let episodes = client.episodes("3pbs", "black-wax")?;

    assert_eq!(
        episodes,
        mock_airnet::expected::episodes()
    );
    Ok(())
}

#[test]
fn test_get_playlist() -> Result<(), Box<dyn std::error::Error>> {
    let server = mock_airnet::start_mock_airnet_server()?;

    let client = airnet::AirnetClient::new(server.base_url());
    let playlist = client.episode_playlist("3pbs", "black-wax", "2025-06-16+11:00:00")?;

    assert_eq!(playlist, mock_airnet::expected::episode_playlist());
    Ok(())
}

#[test]
fn test_fails_when_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let server = mock_airnet::start_mock_airnet_server()?;

    let client = airnet::AirnetClient::new(server.base_url());
    let result = client.program("not-exists", "not-exists");
    // TODO check correct message
    assert!(result.is_err());
    Ok(())
}