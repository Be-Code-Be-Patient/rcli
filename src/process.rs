use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::CsvOpts;

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(opts: CsvOpts) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(opts.input)?;
    let mut players: Vec<Player> = Vec::with_capacity(128);
    for result in reader.deserialize::<Player>() {
        let player = result?;
        players.push(player);
    }

    let json = serde_json::to_string_pretty(&players)?;
    fs::write(opts.output, json)?;
    Ok(())
}
