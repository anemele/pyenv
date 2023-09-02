use rvv::cli;

fn main() {
    cli::run();
}

/* fn main() {
    let matches = App::new("rvv")
        .version("0.1")
        .author("Jason Swift <anemele@outlook.com>")
        .about("Python virtual environments manager")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add(Create) a new venv")
                .arg(
                    Arg::with_name("name")
                        .takes_value(true)
                        .help("The name of the new venv"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        println!("{}", matches.is_present("name"))
    }
} */
