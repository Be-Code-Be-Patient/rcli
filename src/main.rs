use clap::Parser;
use rcli::{
    Base64SubCommand, Opts, Subcommand, TextSignFormat, TextSubCommand, process_base64_decode,
    process_base64_encode, process_csv, process_pwd, process_text_generate, process_text_sign,
    process_text_verify,
};
use std::fs;
use zxcvbn::zxcvbn;

// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            println!("Convert csv: {:?}", opts);
            process_csv(opts)?;
        }
        Subcommand::Pwd(pwd_opts) => {
            // println!("Generate password: {:?}", pwd_opts);
            let password = process_pwd(pwd_opts)?;
            println!("{}", password);
            let result = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", result.score());
        }
        Subcommand::Base64(base64_sub_command) => match base64_sub_command {
            Base64SubCommand::Encode(base64_encode_opts) => {
                // println!("Encode base64: {:?}", base64_encode_opts);
                let encoded = process_base64_encode(base64_encode_opts)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(base64_decode_opts) => {
                // println!("Decode base64: {:?}", base64_decode_opts);
                let decoded = process_base64_decode(base64_decode_opts)?;
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        Subcommand::Text(sub_command) => match sub_command {
            TextSubCommand::Sign(opts) => {
                //println!("Text Sign: {:?}", opts);
                let signed = process_text_sign(opts)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                //println!("Text Verify: {:?}", opts);
                let verified = process_text_verify(opts)?;
                print!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                println!("Text key Generate: {:?}", opts);
                let format = opts.format;
                let output = opts.output.clone();
                let keys = process_text_generate(opts)?;
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
            }
        },
    }
    Ok(())
}
