use std::path::Path;
use std::process::{Command, Stdio};

const VENV_EXE: &str = "virtualenv";

pub(crate) fn exec(venv_path: impl AsRef<Path>, name: &str, version: Option<String>, force: bool) {
    let path = venv_path.as_ref().join(name);

    if path.is_file() || (path.is_dir() && !force) {
        eprintln!("Env with the same name `{name}` exists.");
        return;
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
        eprintln!(
            "Failed to create env `{name}`: {e}\nMaybe `{}` is not in PATH?",
            VENV_EXE
        );
        return;
    };

    #[cfg(target_family = "windows")]
    {
        use std::fs;
        // create idle for Windows
        let _ = fs::write(
            path.join("Scripts\\idle.bat"),
            b"@call %~dp0python.exe -m idlelib %*",
        );
    }
}
