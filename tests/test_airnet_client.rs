use httpmock::prelude::*;
use pbsfm_rss_feed::airnet::types::*;
use pbsfm_rss_feed::airnet;
use std::fs;
use chrono::{NaiveDate};

#[test]
fn test_get_all_programs() -> Result<(), Box<dyn std::error::Error>> {
    let json_response = fs::read_to_string("tests/all-programs.json")?;

    let server = MockServer::start();
    let mock_airnet_server = server.mock(|when, then| {
        when.method("GET").path("/rest/stations/3pbs/programs");
        then.status(200).body(json_response);
    });

    let client = airnet::AirnetClient::new(server.base_url());
    let programs = client.all_programs("3pbs")?;

    assert_eq!(
        programs,
        vec!(
            ProgramDescription{
                slug: None,
                name: String::from("Tomorrow Land"),
                broadcasters: String::from(""),
                grid_description: None,
                archived: true,
                program_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/"),
            },
            ProgramDescription{
                slug: Some(String::from("black-wax")),
                name: String::from("Black Wax"),
                broadcasters: String::from("Adam Rudegeair"),
                grid_description: Some(String::from("Groovin' jazz")),
                archived: false,
                program_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/black-wax"),
            }
        )
    );
    mock_airnet_server.assert();
    Ok(())
}

#[test]
fn test_get_program() -> Result<(), Box<dyn std::error::Error>> {
    let json_response = fs::read_to_string("tests/program.json")?;

    let server = MockServer::start();
    let mock_airnet_server = server.mock(|when, then| {
        when.method("GET").path("/rest/stations/3pbs/programs/my-program");
        then.status(200).body(json_response);
    });

    let client = airnet::AirnetClient::new(server.base_url());
    let program = client.program("3pbs", "my-program")?;

    assert_eq!(
        program,
        ProgramDetails{
            url: None,
            name: String::from("Black Wax"),
            broadcasters: String::from("Adam Rudegeair"),
            description: String::from("Jazz and funk with jazz influence"),
            grid_description: Some(String::from("Groovin' jazz")),
            slug: String::from("black-wax"),
            banner_image_url: String::from("https://banner.jpg"),
            banner_image_small: String::from("https://banner-small.jpg"),
            profile_image_url: String::from("https://profile-img.jpg"),
            profile_image_small: String::from("https://profile-img-small.jpg"),
            episodes_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/black-wax/episodes"),
        }
    );
    mock_airnet_server.assert();
    Ok(())
}

#[test]
fn test_get_episode_list() -> Result<(), Box<dyn std::error::Error>> {
    let json_response = fs::read_to_string("tests/episodes.json")?;

    let server = MockServer::start();
    let mock_airnet_server = server.mock(|when, then| {
        when.method("GET").path("/rest/stations/3pbs/programs/my-program/episodes");
        then.status(200).body(json_response);
    });

    let client = airnet::AirnetClient::new(server.base_url());
    let episodes = client.episodes("3pbs", "my-program")?;

    assert_eq!(
        episodes,
        vec!(
            Episode{
                url: None,
                start: NaiveDate::from_ymd_opt(2025, 6, 16).unwrap()
                    .and_hms_opt(11, 0, 0).unwrap(),
                end: NaiveDate::from_ymd_opt(2025, 6, 16).unwrap()
                    .and_hms_opt(13, 0, 0).unwrap(),
                duration: 7200,
                title: String::from("Interview with Vince Jones and Jacob Collier!"),
                description: None,
                image_url: None,
                episode_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/black-wax/episodes/2025-06-16+11%3A00%3A00"),
            },
            Episode{
                url: Some(String::from("http://url")),
                start: NaiveDate::from_ymd_opt(2025, 8, 25).unwrap()
                    .and_hms_opt(11, 0, 0).unwrap(),
                end: NaiveDate::from_ymd_opt(2025, 8, 25).unwrap()
                    .and_hms_opt(13, 0, 0).unwrap(),
                duration: 7200,
                title: String::from("Paul Grabowsky Feature"),
                description: Some(String::from("some description")),
                image_url: Some(String::from("http://img-url")),
                episode_rest_url: String::from("https://airnet.org.au/rest/stations/3pbs/programs/black-wax/episodes/2025-08-25+11%3A00%3A00"),
            },
        )
    );
    mock_airnet_server.assert();
    Ok(())
}