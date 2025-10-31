use crate::verify_path;
use crate::{
    CmdExecutor, process_text_generate, process_text_sign, process_text_verify, verify_file,
};
use anyhow::anyhow;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign text with private/shared key")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "Verify signature of text")]
    Verify(TextVerifyOpts),
    #[command(name = "generate", about = "Generate a new key")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, default_value = "blake3", value_parser = parse_text_format)]
    pub format: TextSignFormat,
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        //println!("Text Sign: {:?}", opts);
        let signed = process_text_sign(self)?;
        println!("{}", signed);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub signature: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_format)]
    pub format: TextSignFormat,
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        //println!("Text Verify: {:?}", opts);
        let verified = process_text_verify(self)?;
        print!("{}", verified);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_text_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("Text key Generate: {:?}", self);
        let format = self.format;
        let output = self.output.clone();
        let keys = process_text_generate(self)?;
        println!("Keys: {:?}", keys);
        match format {
            TextSignFormat::Blake3 => {
                let name = output.join("blake3.txt");
                fs::write(name, &keys[0])?;
            }
            TextSignFormat::Ed25519 => {
                fs::write(output.join("ed25519.sk"), &keys[0])?;
                fs::write(output.join("ed25519.pk"), &keys[1])?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow!("Unknown format: {}", s)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
