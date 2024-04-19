use std::fs;

mod cli;
mod cmd_add;
mod cmd_list;
mod cmd_remove;
mod cmd_use;
mod consts;
mod utils;

use clap::Parser;

use crate::cli::Cli;
use crate::utils::get_venv_path;

fn main() {
    let Some(venv_path) = get_venv_path() else {
        eprintln!("failed to get HOME dir");
        return;
    };

    if !venv_path.exists() && fs::create_dir(&venv_path).is_err() {
        eprintln!("failed to create dir: {}", venv_path.display());
        return;
    }

    match Cli::parse() {
        Cli::Add {
            name,
            version,
            force,
        } => cmd_add::exec(venv_path, &name, version, force),
        Cli::List => cmd_list::exec(venv_path),
        Cli::Remove { name } => cmd_remove::exec(venv_path, &name),
        Cli::Use { name, pwsh } => cmd_use::exec(venv_path, &name, pwsh),
    };
}
