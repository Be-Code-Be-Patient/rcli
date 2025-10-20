use clap::Parser;
use rcli::{Opts, Subcommand, process_csv};

// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            println!("Input: {}", opts.input);
            println!("Output: {:?}", opts.output); // Changed from {} to {:?}
            println!("Delimiter: {}", opts.delimiter);
            println!("Header: {}", opts.header);
            println!("format: {}", opts.format);

            process_csv(opts)?;
        }
    }
    Ok(())
}
