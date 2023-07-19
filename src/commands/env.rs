use std::error::Error;

use clap::Args;

use crate::DATA_DIR;

#[derive(Args)]
pub struct Command {}

pub fn run(cmd: &Command) -> Result<(), Box<dyn Error>> {
    let env_sh = include_str!("../snippets/env.sh");
    print!(
        "{}",
        env_sh.replace("{data_dir}", &*DATA_DIR.to_str().ok_or("Invalid data dir")?)
    );
    Ok(())
}
