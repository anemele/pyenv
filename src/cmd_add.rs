use crate::get_venv_path;
use std::process::{Command, Stdio};

const VENV_EXE: &str = "virtualenv";

pub fn exec(name: &str, version: Option<String>, force: bool) -> anyhow::Result<()> {
    let path = get_venv_path()?.join(name);

    if path.is_file() || (path.is_dir() && !force) {
        anyhow::bail!("Env with the same name `{name}` exists.");
    }

    let mut cmd = &mut Command::new(VENV_EXE);
    cmd = cmd.arg(path.as_os_str());
    if let Some(ver) = version {
        cmd = cmd.arg("--python").arg(ver);
    }

    // something unnecessary
    cmd = cmd.args(["--no-setuptools", "--no-wheel", "--no-vcs-ignore"]);

    if let Err(e) = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        anyhow::bail!(
            "Failed to create env `{name}`: {e}\nMaybe `{}` is not in PATH?",
            VENV_EXE
        );
    };

    #[cfg(target_family = "windows")]
    {
        use std::fs;
        // create idle for Windows
        fs::write(
            path.join("Scripts\\idle.bat"),
            b"@call %~dp0python.exe -m idlelib %*",
        )?;
    }

    Ok(())
}
