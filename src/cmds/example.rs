use clap::Parser;

use crate::Commands;

#[derive(Parser)]
pub struct CliCommand {}

pub fn match_and_run(commands: &Commands) {
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match commands {
        Commands::EXAMPLE(cmd) => run(cmd),
        _ => {}
    };
}

fn run(cmd: &CliCommand) {
    // code here
}
