use httpmock::prelude::*;
use pbsfm_rss_feed::airnet::types::*;
use pbsfm_rss_feed::airnet;
use std::fs;
// use serde::__private::de::Content::String;
// use crate pbsfm_rss_feed;

/*
  - all programs
    https://airnet.org.au/rest/stations/3pbs/programs
  - program page
    https://airnet.org.au/rest/stations/3pbs/programs/black-wax
  - episodes
    https://airnet.org.au/rest/stations/3pbs/programs/black-wax/episodes

 */

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