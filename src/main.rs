use clap::Parser;
use rcli::{Opts, Subcommand, process_csv, process_pwd};

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
    }
    Ok(())
}
