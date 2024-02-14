use clap::command;

/// The CLI interface for Leagus
fn main() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(leagues::commands())
        .subcommand(participants::commands())
        .get_matches();

    match matches.subcommand() {
        Some((leagues::CMD_NAME, sub_matches)) => leagues::handle_subcommands(&sub_matches),
        Some((participants::CMD_NAME, sub_matches)) => participants::handle_subcommands(&sub_matches),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

/// Module for the league commands
mod leagues {
    use clap::{arg, ArgMatches, Command};
    use leagus::models::League;
    use leagus::persistence::WriteableStore;
    use leagus::persistence::mongo_store::MongoStore;

    pub const CMD_NAME: &str = "leagues";

    pub fn commands() -> Command {
        Command::new(CMD_NAME)
            .about("Commands for managing leagues")
            .subcommand_required(true)
            .subcommand(
                Command::new("create").about("Create a new league").arg(
                    arg!(
                        -n --name <NAME> "Name of new league"
                    )
                    .required(true),
                ),
            )
            .subcommand(
                Command::new("list")
                    .about("List existing leagues")
                    .arg(arg!(
                        -n --name <NAME> "Name of new league"
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
        println!("Created new league: \"{}\"", name);
        let league = League::new(name.to_string());

        //TODO: Store via the persistence layer
        let mut store = MongoStore::new();
        store.create_league(league);
    }

    fn list(_matches: &ArgMatches) {
        println!("Listing existing leagues");
        let store = MongoStore::new();
        let leagues = store.list_leagues();
        for league in leagues {
            println!("League: {:?}", league);
        }
    }
}

/// Module for the participant commands
mod participants {
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
}
