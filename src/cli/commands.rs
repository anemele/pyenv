use clap::Subcommand;
use std::fs;

#[derive(Subcommand)]
pub enum Commands {
    Add { name: String, version: Option<f32> },
    Ls,
    Rm { name: String },
    Env { name: String },
}

pub fn add(name: &String, version: &Option<f32>) {
    println!("name: {}, version: {:?}", name, version)
}

pub fn rm(name: &String) {
    println!("{}", name)
}

pub fn ls(path: &String) {
    let paths = fs::read_dir(&path).unwrap();

    println!("Available envs:");
    for path in paths {
        println!(
            "  {}",
            path.unwrap().path().file_name().unwrap().to_str().unwrap()
        )
    }
}

pub fn env(name: &String) {
    println!("{}", name)
}
