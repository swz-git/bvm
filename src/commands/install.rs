use std::{error::Error, str::FromStr};

use clap::Args;

use crate::{bun_version::BunVersion, DATA_DIR};

#[derive(Args)]
pub struct Command {
    /// Version to install
    /// ex. 0.1.6 or latest
    #[clap(value_parser(BunVersion::from_str))]
    bun_version: BunVersion,
    //
    // TODO: add baseline option for users without avx2
}

// TODO: loop through github release api pages to find version function

pub fn run(cmd: &Command) -> Result<(), Box<dyn Error>> {
    println!("{:?} {:?}", cmd.bun_version, &*DATA_DIR);
    Ok(())
}
