use std::{fs, path::Path};

use clap::Parser;
use dialoguer::Confirm;

use crate::{utils::get_version_bin, Commands};

#[derive(Parser)]
pub struct CliCommand {
    /// Version to uninstall
    /// ex. 0.1.6 or latest
    #[clap(value_parser)]
    version: String,
}

pub fn match_and_run(commands: &Commands) {
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match commands {
        Commands::Uninstall(cmd) => run(cmd),
        _ => {}
    };
}

fn run(cmd: &CliCommand) {
    let bin = get_version_bin(&cmd.version, true)
        .expect("Couldn't find that version installed using bvm");
    let dir = Path::new(&bin).parent().unwrap();
    let dirname = dir.file_name().unwrap().to_str().unwrap();
    let doit = Confirm::new()
        .with_prompt(format!("Are you sure you want to uninstall {}", dirname))
        .interact()
        .unwrap();
    if doit {
        fs::remove_dir_all(dir).unwrap();
        println!("Removed {}", dirname)
    } else {
        println!("Did not remove {}", dirname)
    }
}
