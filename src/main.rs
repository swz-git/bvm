use std::{error::Error, fs, path::PathBuf};

use clap::{Parser, Subcommand};

#[macro_use]
extern crate lazy_static;

pub mod bun_version;
mod commands;

lazy_static! {
    // ~/.local/share on linux
    #[derive(Debug)]
    pub static ref DATA_DIR: PathBuf = {
        directories::BaseDirs::new()
            .unwrap()
            .data_dir().join("bvm").to_path_buf()
    };
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Env(commands::env::Command),
    Install(commands::install::Command),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // TODO: Add warn if env not activated

    // create DATA_DIR/bin and DATA_DIR/versions for future use
    if !DATA_DIR.exists() {
        fs::create_dir(&*DATA_DIR)?
    }
    if !DATA_DIR.join("bin").exists() {
        fs::create_dir(&*DATA_DIR.join("bin"))?
    }
    if !DATA_DIR.join("versions").exists() {
        fs::create_dir(&*DATA_DIR.join("versions"))?
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Env(cmd) => commands::env::run(cmd)?,
        Commands::Install(cmd) => commands::install::run(cmd)?,
    };

    Ok(())
}
