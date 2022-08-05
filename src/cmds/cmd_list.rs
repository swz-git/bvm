use clap::Parser;

use crate::{utils::get_available_versions, Commands};

#[derive(Parser)]
pub struct CliCommand {}

pub fn match_and_run(commands: &Commands) {
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match commands {
        Commands::List(cmd) => run(cmd),
        _ => {}
    };
}

fn run(_cmd: &CliCommand) {
    println!("Installed versions:");
    let versions = get_available_versions();
    for version in versions {
        println!("- {}", version)
    }
}
