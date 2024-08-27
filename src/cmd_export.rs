use anyhow::anyhow;

use crate::consts::PY_BIN_DIR;
use crate::get_venv_path;
use crate::manifest::{Env, EnvManifest};
use crate::utils::is_valid_env;

use std::fs::{self, read_dir};
use std::process::{Command, Stdio};

#[inline]
fn export_filename() -> String {
    const PREFIX: &str = "pyvm_manifest";
    let now = chrono::Local::now().date_naive();
    format!("{PREFIX}_{now}.toml")
}

pub fn exec(output: Option<String>) -> anyhow::Result<()> {
    let s = export_library()?;
    let outpath = output.unwrap_or(export_filename());
    if fs::write(&outpath, s).is_err() {
        return Err(anyhow!("failed to write manifest."));
    }

    println!("manifest has been written at {}", outpath);
    Ok(())
}

fn export_library() -> anyhow::Result<String> {
    let venv_path = get_venv_path()?;

    let sep = if cfg!(windows) {
        "\r\n"
    } else if cfg!(darwin) {
        "\r"
    } else {
        "\n"
    };

    let mut pipe = vec![];
    for path in read_dir(&venv_path)? {
        let Ok(dir) = path else {
            continue;
        };
        let path = dir.path();
        if !is_valid_env(&path) {
            continue;
        }
        let Ok(name) = dir.file_name().into_string() else {
            continue;
        };

        let py = path.join(PY_BIN_DIR).join("python");
        let Ok(child1) = Command::new(py)
            .arg("--version")
            .stdout(Stdio::piped())
            .spawn()
        else {
            continue;
        };
        let pip = path.join(PY_BIN_DIR).join("pip");
        let Ok(child2) = Command::new(pip)
            .arg("freeze")
            .stdout(Stdio::piped())
            .spawn()
        else {
            continue;
        };
        pipe.push((name, child1, child2));
    }

    let mut vec = vec![];
    for (name, child1, child2) in pipe {
        let Ok(output) = child1.wait_with_output() else {
            continue;
        };
        let Ok(output) = String::from_utf8(output.stdout) else {
            continue;
        };
        let Some(ver) = output.strip_prefix("Python") else {
            continue;
        };
        let Ok(output) = child2.wait_with_output() else {
            continue;
        };
        let Ok(output) = String::from_utf8(output.stdout) else {
            continue;
        };
        let output = output
            .trim_end()
            .split(sep)
            .map(|s| s.to_string())
            .collect();
        vec.push(Env {
            name,
            ver: ver.trim().to_string(),
            libs: output,
        });
    }
    // dbg!(&vec);

    let s = toml::to_string(&EnvManifest { env: vec })?;
    Ok(s)
}
