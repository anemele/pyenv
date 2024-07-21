use std::process::Command;

use crate::{consts::PY_BIN_DIR, get_venv_path, manifest::EnvManifest};

fn read_manifest(manifest: &str) -> Option<EnvManifest> {
    let Ok(s) = std::fs::read_to_string(&manifest) else {
        eprintln!("failed to read manifest file: {}", manifest);
        return None;
    };
    let Ok(t) = toml::from_str(&s) else {
        eprintln!("failed to load manifest");
        return None;
    };

    Some(t)
}

pub fn exec(manifest: String) {
    let Some(table) = read_manifest(&manifest) else {
        return;
    };
    // dbg!(&t);

    let venv_path = get_venv_path();
    let mut children = vec![];
    for env in table.env {
        crate::cmd_add::exec(&env.name, Some(env.ver), false);
        let pip = venv_path.join(env.name).join(PY_BIN_DIR).join("pip");
        let child = Command::new(pip).arg("install").args(env.libs).spawn();
        children.push(child);
    }

    for child in children {
        let _ = child.and_then(|mut c| c.wait());
    }
}
