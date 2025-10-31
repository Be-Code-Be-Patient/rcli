use std::{fmt::Display, str::FromStr};

use clap::Parser;

use crate::{CmdExecutor, process_base64_decode, process_base64_encode, verify_file};

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

impl CmdExecutor for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    UrlSafe,
    Standard,
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, default_value = "standard", value_parser = parse_base64_format)]
    pub format: Base64Format,
}

impl CmdExecutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // println!("Encode base64: {:?}", base64_encode_opts);
        let encoded = process_base64_encode(self)?;
        println!("{}", encoded);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, default_value = "standard", value_parser = parse_base64_format)]
    pub format: Base64Format,
}

impl CmdExecutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // println!("Decode base64: {:?}", base64_decode_opts);
        let decoded = process_base64_decode(self)?;
        let decoded = String::from_utf8(decoded)?;
        println!("{}", decoded);
        Ok(())
    }
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "urlsafe" => Ok(Base64Format::UrlSafe),
            "standard" => Ok(Base64Format::Standard),
            _ => anyhow::bail!("Invalid base64 format: {}", s),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::UrlSafe => "urlsafe",
            Base64Format::Standard => "standard",
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
