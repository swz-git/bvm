use std::{env, fs, path::Path};

use clap::Parser;

use crate::{
    utils::{get_current_version, get_data_dir, get_version_bin, parse_version_string},
    Commands,
};

#[derive(Parser)]
pub struct CliCommand {
    #[clap(value_parser)]
    /// Version to use (global)
    version: Option<String>,
}

pub fn match_and_run(commands: &Commands) {
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match commands {
        Commands::Use(cmd) => run(cmd),
        _ => {}
    };
}

fn print_version(symlink_path: &Path) {
    let version = get_current_version(symlink_path);
    if version.is_none() {
        println!("No version of bun is activated using bvm.");
    } else {
        println!("Using bun version {}", version.unwrap());
    }
}

fn run(cmd: &CliCommand) {
    if env::var("BVM_ACTIVATED").unwrap_or(String::from("")) != "1" {
        println!("WARNING: You haven't added `eval \"$(bvm env)\"` to your shell init yet, you won't be able to use bun through bvm.\n")
    }
    let symlink_dir = Path::new(&get_data_dir()).join("bin");
    let symlink_pathbuf = Path::new(&get_data_dir()).join("bin/bun");
    let symlink_path = symlink_pathbuf.as_path();
    match &cmd.version {
        None => {
            print_version(symlink_path);
            return;
        }
        _ => {}
    }
    let bin = get_version_bin(&cmd.version.as_ref().unwrap(), false).expect(&*format!(
        "Couldn't find version installed: {}",
        parse_version_string(&cmd.version.as_ref().unwrap(), false)
    ));
    if !symlink_dir.exists() {
        fs::create_dir_all(symlink_dir).unwrap();
    }
    if symlink_path.exists() && symlink_path.is_symlink() {
        fs::remove_file(symlink_path).unwrap();
    }
    std::os::unix::fs::symlink(&bin, symlink_path).expect("Failed to create symlink");

    print_version(symlink_path);
}
