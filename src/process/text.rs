use crate::{
    PwdOpts, TextKeyGenerateOpts, TextSignFormat, TextSignOpts, TextVerifyOpts, get_reader,
    process_pwd,
};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::io::Read;
use std::path::Path;

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = process_pwd(PwdOpts {
            length: 32,
            uppercase: true,
            lowercase: true,
            numbers: true,
            symbols: true,
        })?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

pub struct Ed255195Signer {
    key: SigningKey,
}

impl Ed255195Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Ed255195Signer::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Ed255195Signer {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Ed255195Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk.to_bytes().to_vec()])
    }
}

pub struct Ed255195Verifier {
    key: VerifyingKey,
}

impl Ed255195Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        let verifier = Ed255195Verifier::new(key);
        Ok(verifier)
    }

    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed255195Verifier {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes() == sig)
    }
}

impl TextSign for Ed255195Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerify for Ed255195Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&sig[..64]).try_into()?;
        let signature = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

pub fn process_text_sign(opts: TextSignOpts) -> anyhow::Result<String> {
    let mut reader = get_reader(&opts.input)?;
    let signed = match opts.format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(&opts.key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed255195Signer::load(opts.key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = BASE64_URL_SAFE_NO_PAD.encode(&signed);
    Ok(signed)
}

pub fn process_text_verify(opts: TextVerifyOpts) -> anyhow::Result<bool> {
    let mut reader = get_reader(&opts.input)?;
    let sig = URL_SAFE_NO_PAD.decode(opts.signature)?;
    let verified = match opts.format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(&opts.key)?;
            signer.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => Ed255195Verifier::load(opts.key)?.verify(&mut reader, &sig)?,
    };
    Ok(verified)
}

pub fn process_text_generate(opts: TextKeyGenerateOpts) -> anyhow::Result<Vec<Vec<u8>>> {
    match opts.format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed255195Signer::generate(),
    }
}
