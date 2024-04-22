use crate::utils::is_valid_env;
use std::path::Path;

pub(crate) fn exec(venv_path: impl AsRef<Path>, name: &str) {
    let path = venv_path.as_ref().join(name);

    if !path.exists() {
        eprintln!("No env `{name}` exists.");
        return;
    }

    if !is_valid_env(&path) {
        eprintln!("Invalid env `{name}`");
        return;
    }

    #[cfg(target_family = "windows")]
    {
        use std::process::Command;
        if let Err(e) = Command::new("cmd")
            .arg("/c")
            .arg("start cmd /k")
            .arg(path.join("Scripts/activate.bat"))
            .status()
        {
            eprintln!("Failed to activate env `{name}`:\n{e}")
        }
    }

    #[cfg(target_family = "unix")]
    {
        // eprintln!("NOT support on *nix");
        // eprintln!("use `source` command instead");
        println!(
            "source {}/{}/bin/activate",
            venv_path.as_ref().display(),
            name
        )
    }
}
