use clap::Parser;

#[derive(Parser)]
#[clap(
name = "pyvm",
version,
author,
about = "Python Virtual env Manager",
long_about = None,
)]
pub(crate) enum Cli {
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
