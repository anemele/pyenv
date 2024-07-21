use std::{fs, path::PathBuf};

mod cli;
mod cmd_add;
mod cmd_export;
mod cmd_import;
mod cmd_list;
mod cmd_remove;
mod cmd_use;
mod consts;
mod utils;

use clap::Parser;

use cli::Cli;

pub fn get_venv_path() -> PathBuf {
    let Some(venv_path) = utils::get_venv_path() else {
        panic!("failed to get HOME dir");
    };

    if !venv_path.exists() && fs::create_dir(&venv_path).is_err() {
        panic!("failed to create dir: {}", venv_path.display());
    }

    venv_path
}

fn main() {
    match Cli::parse() {
        Cli::Add {
            name,
            version,
            force,
        } => cmd_add::exec(&name, version, force),
        Cli::List => cmd_list::exec(),
        Cli::Remove { name } => cmd_remove::exec(&name),
        Cli::Use { name } => cmd_use::exec(&name),
        Cli::Export { output } => cmd_export::exec(output),
        Cli::Import { manifest } => cmd_import::exec(manifest),
    };
}
