use clap::Subcommand;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::utils;

#[derive(Subcommand)]
pub enum Commands {
    Add { name: String },
    Ls,
    Rm { name: String },
    Env { name: String },
}

pub fn create(path: &Path) {
    let output = Command::new("virtualenv")
        .arg(path.as_os_str())
        .arg("--activators")
        .arg("batch,powershell")
        .arg("--no-setuptools")
        .arg("--no-wheel")
        .output()
        .expect("Failed to create new env");
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{output_str}");
}

pub fn remove(path: &Path, name: &String) {
    match fs::remove_dir_all(path) {
        Ok(_) => println!("Removed env `{name}`"),
        Err(_) => println!("Failed to remove `{name}`"),
    };
}

pub fn list(path: &String) {
    let paths = fs::read_dir(&path).unwrap();

    println!("Available envs:");
    for path in paths {
        match path {
            Ok(dir) => {
                if utils::is_valid_env(dir.path().as_path()) {
                    println!("  {}", dir.file_name().into_string().unwrap())
                }
            }
            Err(_) => todo!(),
        }
    }
}

pub fn activate(path: &Path) {
    let _ = Command::new("cmd")
        .arg("/c")
        .arg("start cmd /k")
        .arg(path.join("Scripts/activate"))
        .spawn()
        // .output()
        .expect("Failed to activate env");
}
