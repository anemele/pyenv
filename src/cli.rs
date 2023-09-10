use std::path::Path;

use crate::commands::{self, Commands};
use crate::utils;
use clap::Parser;

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
    let venv_root = Path::new(&venv_path);

    let cli = Cli::parse();
    // println!("JAVA Home: {:?}", cli.path);
    match &cli.command {
        Commands::Add { name } => {
            let vp = venv_root.join(name);
            if vp.is_file() {
                eprintln!("File with the same name exists.")
            } else {
                commands::create(&vp, name);
            }
        }
        Commands::Ls => commands::list(&venv_path),
        Commands::Rm { name } => {
            let vp = venv_root.join(name);
            if !vp.exists() {
                eprintln!("No env `{name}` exists.")
            } else {
                commands::remove(&vp, name);
            }
        }
        Commands::Env { name } => {
            let vp = venv_root.join(name);
            if !vp.exists() {
                eprintln!("No env `{name}` exists.")
            } else {
                commands::activate(&vp, name);
            }
        }
    }
}
