use std::io::Read;

use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};

use crate::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, get_reader};

pub fn process_base64_encode(opts: Base64EncodeOpts) -> anyhow::Result<String> {
    let mut reader: Box<dyn Read> = get_reader(&opts.input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match opts.format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn process_base64_decode(opts: Base64DecodeOpts) -> anyhow::Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = get_reader(&opts.input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match opts.format {
        Base64Format::Standard => STANDARD.decode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf),
    }?;
    Ok(decoded)
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
