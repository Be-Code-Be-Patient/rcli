mod base64;
mod csv;
mod pwd;

use std::path::Path;

pub use base64::*;
pub use csv::*;
pub use pwd::*;

use clap::{Parser, command};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(
        name = "csv",
        about = "Show CSV, or convert a CSV file to other formats"
    )]
    Csv(CsvOpts),

    #[command(name = "pwd", about = "Generate a random password")]
    Pwd(PwdOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),
}

pub fn verify_input_file(file_name: &str) -> Result<String, String> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("Input file does not exist.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(
            verify_input_file("nonexistent"),
            Err("Input file does not exist.".into())
        );
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
