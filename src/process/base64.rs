use std::{fs::File, io::Read};

use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};

use crate::{Base64DecodeOpts, Base64EncodeOpts, Base64Format};

pub fn process_base64_encode(opts: Base64EncodeOpts) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = get_reader(&opts.input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match opts.format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_base64_decode(opts: Base64DecodeOpts) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = get_reader(&opts.input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match opts.format {
        Base64Format::Standard => STANDARD.decode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf),
    }?;
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let opts = Base64EncodeOpts {
            input: "Cargo.toml".to_string(),
            format: Base64Format::Standard,
        };
        let result = process_base64_encode(opts);
        assert!(result.is_ok());
    }

    #[test]
    fn test_base64_decode() {
        let opts = Base64DecodeOpts {
            input: "fixtures/temp.b64".to_string(),
            format: Base64Format::UrlSafe,
        };
        let result = process_base64_decode(opts);
        assert!(result.is_ok());
    }
}
