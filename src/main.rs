use std::{fs, path::PathBuf};

mod cli;
mod cmd_add;
mod cmd_export;
mod cmd_import;
mod cmd_list;
mod cmd_remove;
mod cmd_use;
mod consts;
mod manifest;
mod utils;

use clap::Parser;

use cli::Cli;

pub fn get_venv_path() -> anyhow::Result<PathBuf> {
    let venv_path = utils::get_venv_path()?;
    if !venv_path.exists() {
        fs::create_dir(&venv_path)?;
    }

    Ok(venv_path)
}

fn main() -> anyhow::Result<()> {
    match Cli::parse() {
        Cli::Add {
            name,
            version,
            force,
        } => cmd_add::exec(&name, version, force)?,
        Cli::List => cmd_list::exec()?,
        Cli::Remove { name } => cmd_remove::exec(&name)?,
        Cli::Use { name } => cmd_use::exec(&name)?,
        Cli::Export { output } => cmd_export::exec(output)?,
        Cli::Import { manifest } => cmd_import::exec(&manifest)?,
    }

    Ok(())
}
