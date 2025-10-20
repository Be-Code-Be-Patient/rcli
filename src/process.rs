use std::fs;

use chrono::Utc;
use chrono_tz::Asia;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use toml::map::Map;

use crate::{CsvOpts, OutputFormat};

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
    let output = if let Some(output) = opts.output {
        output.clone()
    } else {
        let now = Utc::now();
        let timestamp = now
            .with_timezone(&Asia::Shanghai)
            .format("%Y%m%d_%H%M%S")
            .to_string();
        format!("{}_output.{}", timestamp, opts.format)
    };
    let mut reader = Reader::from_path(opts.input)?;
    let mut records = Vec::with_capacity(128);
    let mut toml_records = Vec::new();

    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        match opts.format {
            OutputFormat::Toml => {
                let mut map = Map::new();
                for (i, field) in record.iter().enumerate() {
                    map.insert(
                        headers[i].to_string(),
                        toml::Value::String(field.to_string()),
                    );
                }
                toml_records.push(toml::Value::Table(map));
            }
            _ => {
                let json_value = headers.iter().zip(record.iter()).collect::<Value>();
                records.push(json_value);
            } // Do nothing for other formats
        }
    }

    let content = match opts.format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
        OutputFormat::Toml => {
            let mut root_table = Map::new();
            root_table.insert("players".to_string(), toml::Value::Array(toml_records));
            toml::to_string_pretty(&toml::Value::Table(root_table))?
        }
    };

    fs::write(output, content)?;
    Ok(())
}
