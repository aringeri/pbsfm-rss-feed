// use chrono::NaiveDate;
// use rss_gen::{macro_generate_rss, macro_write_element};
// use quick_xml::writer::Writer;
// use std::io::{Error};
// use std::fs::File;
// use std::io::BufWriter;

mod mock_airnet;

#[test]
fn test_convert_to_rss() -> Result<(), Box<dyn std::error::Error>> {
    let program = mock_airnet::expected::single_program();
    let episodes = mock_airnet::expected::episodes();

    let _rss_feed = pbsfm_rss_feed::convert_to_rss(program, episodes)?;
    // let writer = Writer::new(BufWriter::new(File::create("output.xml")?));
    // let result: Result<Writer<BufWriter<File>>, Error> = macro_generate_rss!(writer, rss_feed);
    // assert!(result.is_ok());
    // assert_eq!(rss_feed.title, program.name);
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