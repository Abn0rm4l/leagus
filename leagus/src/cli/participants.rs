use clap::{arg, ArgMatches, Command};

pub const CMD_NAME: &str = "participants";

pub fn commands() -> Command {
    Command::new(CMD_NAME)
        .about("Commands for managing participants")
        .subcommand_required(true)
        .subcommand(
            Command::new("create")
                .about("Create a new participants")
                .arg(
                    arg!(
                        -n --name <NAME> "Name of new participants"
                    )
                    .required(true),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List existing participants")
                .arg(arg!(
                    -n --name <NAME> "Name of participant"
                )),
        )
}

pub fn handle_subcommands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("create", sub_matches)) => create(sub_matches),
        Some(("list", sub_matches)) => list(sub_matches),
        _ => unreachable!("Must specify a subcommand"),
    }
}

fn create(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").expect("required");
    println!("Created new participant: \"{}\"", name)
}

fn list(_matches: &ArgMatches) {
    println!("Listing existing participants")
}
