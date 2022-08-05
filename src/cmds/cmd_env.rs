use clap::Parser;
use tinytemplate::TinyTemplate;

use crate::{utils::get_data_dir, Commands};

#[derive(Parser)]
pub struct CliCommand {}

pub fn match_and_run(commands: &Commands) {
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match commands {
        Commands::Env(cmd) => run(cmd),
        _ => {}
    };
}

const ENV_SNIPPET: &str = include_str!("../snippets/env.sh");

#[derive(Serialize)]
struct Context {
    data_dir: String,
}

fn run(_cmd: &CliCommand) {
    let mut tt = TinyTemplate::new();
    tt.add_template("env_snippet", ENV_SNIPPET).unwrap();

    let context = Context {
        data_dir: get_data_dir(),
    };

    let rendered = tt.render("env_snippet", &context).unwrap();
    print!("{}", rendered);
}
