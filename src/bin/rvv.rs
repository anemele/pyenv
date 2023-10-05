use clap::Parser;
use rvv::commands;
use rvv::utils;
use std::path::Path;
use std::process;

#[derive(Parser)]
#[clap(
name = "rvv",
version,
author,
about = "Rust-implemented Python Virtual enVironment manager",
long_about = None,
)]

enum RVV {
    Add {
        name: String,
        version: Option<f32>,
        #[arg(long)]
        // #[arg(short, long)]
        overwrite: bool,
    },
    #[clap(alias = "ls")]
    List,
    #[clap(alias = "rm")]
    Remove {
        name: String,
    },
    Use {
        name: String,
    },
}

fn main() {
    let venv_path = utils::get_venv_path();
    let venv_path = Path::new(&venv_path);
    if !venv_path.exists() {
        return;
    }

    let code = match RVV::parse() {
        RVV::Add {
            name,
            version,
            overwrite,
        } => commands::create(&venv_path, &name, version, overwrite),
        RVV::List => commands::list(&venv_path),
        RVV::Remove { name } => commands::remove(&venv_path, &name),
        RVV::Use { name } => commands::activate(&venv_path, &name),
    };

    process::exit(code)
}
