#[macro_use]
extern crate serde_derive;
extern crate tinytemplate;

use clap::{Parser, Subcommand};
use std::env;

mod cmds;
pub mod utils;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install a bun version
    Install(cmds::cmd_install::CliCommand),
    Use(cmds::cmd_use::CliCommand),
    Env(cmds::cmd_env::CliCommand),
    List(cmds::cmd_list::CliCommand),
    Uninstall(cmds::cmd_uninstall::CliCommand),
}

#[tokio::main]
async fn main() {
    if env::consts::OS != "linux" {
        panic!("This program is only supported on Linux, other OSs are planned");
    }
    let cli = Cli::parse();

    cmds::cmd_install::match_and_run(&cli.command).await;
    cmds::cmd_use::match_and_run(&cli.command);
    cmds::cmd_env::match_and_run(&cli.command);
    cmds::cmd_list::match_and_run(&cli.command);
    cmds::cmd_uninstall::match_and_run(&cli.command);
}
