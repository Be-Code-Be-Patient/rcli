mod base64;
mod csv;
mod http;
mod pwd;
mod text;

use std::path::{Path, PathBuf};

pub use base64::*;
pub use csv::*;
pub use http::*;
pub use pwd::*;
pub use text::*;

use clap::{Parser, command};
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Subcommand {
    #[command(
        name = "csv",
        about = "Show CSV, or convert a CSV file to other formats"
    )]
    Csv(CsvOpts),

    #[command(name = "pwd", about = "Generate a random password")]
    Pwd(PwdOpts),

    #[command(subcommand, about = "Base64 encoding and decoding")]
    Base64(Base64SubCommand),

    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),

    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

pub fn verify_file(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("Input file does not exist.")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(
            verify_file("nonexistent"),
            Err("Input file does not exist.")
        );
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
