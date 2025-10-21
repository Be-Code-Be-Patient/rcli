use clap::Parser;
use rcli::{
    Base64SubCommand, Opts, Subcommand, process_base64_decode, process_base64_encode, process_csv,
    process_pwd,
};

// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            println!("Convert csv: {:?}", opts);
            process_csv(opts)?;
        }
        Subcommand::Pwd(pwd_opts) => {
            println!("Generate password: {:?}", pwd_opts);
            process_pwd(pwd_opts)?;
        }
        Subcommand::Base64(base64_sub_command) => match base64_sub_command {
            Base64SubCommand::Encode(base64_encode_opts) => {
                // println!("Encode base64: {:?}", base64_encode_opts);
                process_base64_encode(base64_encode_opts)?;
            }
            Base64SubCommand::Decode(base64_decode_opts) => {
                // println!("Decode base64: {:?}", base64_decode_opts);
                process_base64_decode(base64_decode_opts)?;
            }
        },
    }
    Ok(())
}
