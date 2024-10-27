use std::{fs, process::Command};

use crate::{consts::PY_BIN_DIR, get_venv_path, manifest::EnvManifest};

fn read_manifest(manifest: &str) -> anyhow::Result<EnvManifest> {
    let s = fs::read_to_string(manifest)?;
    let t = toml::from_str(&s)?;

    Ok(t)
}

pub fn exec(manifest: &str) -> anyhow::Result<()> {
    let table = read_manifest(manifest)?;
    // dbg!(&t);

    let venv_path = get_venv_path()?;
    let mut children = vec![];
    for env in table.env {
        if crate::cmd_add::exec(&env.name, Some(env.ver), false).is_err() {
            continue;
        };
        let pip = venv_path.join(env.name).join(PY_BIN_DIR).join("pip");
        // dbg!(&env.libs);
        if env.libs.is_empty() {
            continue;
        }
        let child = Command::new(pip).arg("install").args(env.libs).spawn();
        children.push(child);
    }

    for child in children {
        child.and_then(|mut c| c.wait())?;
    }

    Ok(())
}
