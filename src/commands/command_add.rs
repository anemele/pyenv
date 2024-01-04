use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

const VENV_EXE: &str = "virtualenv";

pub fn create(venv_path: &Path, name: &String, version: Option<String>, force: bool) {
    let path = venv_path.join(name);

    if path.is_file() || (path.is_dir() && !force) {
        eprintln!("Env with the same name `{name}` exists.");
        return;
    }

    let mut cmd = Command::new(VENV_EXE);
    let mut cmd = cmd.arg(path.as_os_str());
    if let Some(ver) = version {
        cmd = cmd.arg("--python").arg(ver);
    }

    // The following lines are customized settings
    cmd = cmd.args(["--no-setuptools", "--no-wheel", "--no-vcs-ignore"]);
    if cfg!(windows) {
        // bash,batch,fish,nushell,powershell,python
        cmd = cmd.args(["--activators", "batch,powershell"])
    }

    let ok = match cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(status) => {
            if let Some(code) = status.code() {
                code == 0
            } else {
                false
            }
        }

        Err(e) => {
            eprintln!(
                "Failed to create env `{name}`: {e}\nMaybe `{}` is not in PATH?",
                VENV_EXE
            );
            return;
        }
    };

    if cfg!(windows) && ok && path.exists() {
        // create idle for Windows
        let idle = path.join("Scripts/idle.bat");
        if let Ok(mut file) = File::create(idle) {
            let _ = file.write_all(b"@call %~dp0python.exe -m idlelib %*");
        }
    }
}
