use crate::{CmdExecutor, process_pwd};
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct PwdOpts {
    #[arg(short, long, default_value = "16")]
    pub length: u8,

    #[arg(long, default_value = "true")]
    pub uppercase: bool,

    #[arg(long, default_value = "true")]
    pub lowercase: bool,

    #[arg(long, default_value = "true")]
    pub numbers: bool,

    #[arg(long, default_value = "true")]
    pub symbols: bool,
}

impl CmdExecutor for PwdOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // println!("Generate password: {:?}", pwd_opts);
        let password = process_pwd(self)?;
        println!("{}", password);
        let result = zxcvbn(&password, &[]);
        eprintln!("Password strength: {}", result.score());
        Ok(())
    }
}
