use super::utils;
use clap::Parser;
use commands::Commands;

mod commands;
mod parser;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    // #[arg(env = "JAVA_HOME")]
    // path: String,
}

pub fn run() {
    let venv_path = utils::get_venv_path();
    if !utils::exists(&venv_path) {
        return;
    }

    let cli = Cli::parse();
    // println!("JAVA Home: {:?}", cli.path);
    match &cli.command {
        Commands::Add { name, version } => {
            commands::add(name, version);
        }
        Commands::Ls => commands::ls(&venv_path),
        Commands::Rm { name } => {
            commands::rm(name);
        }
        Commands::Env { name } => {
            commands::env(name);
        }
    }
}
