pub mod types;

use reqwest::blocking::Client;
use reqwest::Error;
use crate::airnet::types::{Episode, ProgramDescription, ProgramDetails};

pub struct AirnetClient {
    client: Client,
    base_url: String
}

impl AirnetClient {

    pub fn new(base_url: String) -> Self {
        Self{
            client: Client::new(),
            base_url
        }
    }

    pub fn all_programs(&self, station: &str) -> Result<Vec<ProgramDescription>, Error> {
        let req_url = format!(
            "{base_url}/rest/stations/{station}/programs",
            base_url = self.base_url,
            station = station,
        );

        let response = self.client.get(req_url).send()?;

        let result: Vec<ProgramDescription> = response.json()?;
        Ok(result)
    }

    pub fn program(&self, station: &str, program: &str) -> Result<ProgramDetails, Error> {
        let req_url = format!(
            "{base_url}/rest/stations/{station}/programs/{program}",
            base_url = self.base_url,
            station = station,
            program = program,
        );

        self.client.get(req_url).send()?.json()
    }

    pub fn episodes(&self, station: &str, program: &str) -> Result<Vec<Episode>, Error> {
        let req_url = format!(
            "{base_url}/rest/stations/{station}/programs/{program}/episodes",
            base_url = self.base_url,
            station = station,
            program = program,
        );

        self.client.get(req_url).send()?.json()
    }
}