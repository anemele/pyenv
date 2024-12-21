use crate::get_venv_path;
use crate::utils::is_valid_env;

pub fn exec(name: &str) -> anyhow::Result<()> {
    let path = get_venv_path()?.join(name);

    if !path.exists() {
        anyhow::bail!("No env `{name}` exists.");
    }

    if !is_valid_env(&path) {
        anyhow::bail!("Invalid env `{name}`");
    }

    #[cfg(target_family = "windows")]
    {
        let activator = path.join("Scripts\\activate");
        println!("{}", activator.display());
    }

    #[cfg(target_family = "unix")]
    {
        // eprintln!("NOT support on *nix");
        // eprintln!("use `source` command instead");
        println!("source {}/{}/bin/activate", path.as_ref().display(), name);
    }

    Ok(())
}
