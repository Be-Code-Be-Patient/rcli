use clap::Parser;
use rcli::{Opts, Subcommand, process_csv};

// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            println!("Input: {}", opts.input);
            println!("Output: {}", opts.output);
            println!("Delimiter: {}", opts.delimiter);
            println!("Header: {}", opts.header);

            process_csv(opts)?;
        }
    }
    Ok(())
}
