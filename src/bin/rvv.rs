use clap::Parser;
use rvv::commands;
use rvv::utils;
use std::path::Path;

#[derive(Parser)]
#[clap(
name = "rvv",
version,
author,
about = "Rust-implemented Python Virtual enVironment manager",
long_about = None,
)]

enum Cli {
    Add {
        #[arg(help = "env name")]
        name: String,
        #[arg(help = "Python version")]
        version: Option<String>,
        #[arg(short, long, help = "overwrite an existing env")]
        force: bool,
    },
    #[clap(alias = "ls")]
    List,
    #[clap(alias = "rm")]
    Remove {
        #[clap(required = true, help = "env name, 1+")]
        name: Vec<String>,
    },
    Use {
        #[arg(help = "env name")]
        name: String,
        #[arg(short, long, default_value_t = false, help = "use PowerShell")]
        pwsh: bool,
    },
}

fn main() {
    let venv_path = utils::get_venv_path();
    let venv_path = Path::new(&venv_path);
    if !venv_path.exists() {
        return;
    }

    match Cli::parse() {
        Cli::Add {
            name,
            version,
            force,
        } => commands::create(venv_path, &name, version, force),
        Cli::List => commands::list(venv_path),
        Cli::Remove { name: names } => commands::remove(venv_path, &names),
        Cli::Use { name, pwsh } => commands::activate(venv_path, &name, pwsh),
    };
}

