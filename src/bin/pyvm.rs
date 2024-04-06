use clap::Parser;
use pyvm::cmd;
use pyvm::utils::get_venv_path;
use std::fs;

#[derive(Parser)]
#[clap(
name = "pyvm",
version,
author,
about = "Python Virtual env Manager",
long_about = None,
)]
enum Cli {
    /// Create a new env
    Add {
        #[arg(help = "env name")]
        name: String,
        #[arg(short, long, help = "Python version")]
        version: Option<String>,
        #[arg(
            short,
            long,
            default_value_t = false,
            help = "overwrite an existing env"
        )]
        force: bool,
    },

    /// List all envs
    #[clap(alias = "ls")]
    List,

    /// Remove an existing env
    #[clap(alias = "rm")]
    Remove {
        #[arg(help = "env name")]
        name: String,
    },

    /// Activate an existing env
    Use {
        #[arg(help = "env name")]
        name: String,
        #[arg(short, long, default_value_t = false, help = "use PowerShell v7+")]
        pwsh: bool,
    },
}

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
        } => cmd::create(venv_path, &name, version, force),
        Cli::List => cmd::list(venv_path),
        Cli::Remove { name } => cmd::remove(venv_path, &name),
        Cli::Use { name, pwsh } => cmd::activate(venv_path, &name, pwsh),
    };
}
