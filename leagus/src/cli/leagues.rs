use clap::{arg, ArgMatches, Command};
use leagus::models::League;
use leagus::persistence::sync::{mongo_store::MongoStore, WriteableStore};

pub const CMD_NAME: &str = "leagues";

pub fn commands() -> Command {
    Command::new(CMD_NAME)
        .about("Commands for managing leagues")
        .subcommand_required(true)
        .subcommand(
            Command::new("create")
                .about("Create a new league")
                .arg(
                    arg!(
                        -n --name <NAME> "Name of new league"
                    )
                    .required(true),
                )
                .arg(arg!(
                    -d --description <DESCRIPTION> "Description of the new league"
                )),
        )
        .subcommand(
            Command::new("list")
                .about("List existing leagues")
                .arg(arg!(
                    -n --name <NAME> "Name of new league"
                )),
        )
}

/// Delegate subcommands of the league command
pub fn handle_subcommands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("create", sub_matches)) => create(sub_matches),
        Some(("list", sub_matches)) => list(sub_matches),
        _ => unreachable!("Must specify a subcommand"),
    }
}

/// Create a new league
fn create(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").expect("required");

    let default_description = String::default();
    let description = matches
        .get_one::<String>("description")
        .unwrap_or(&default_description);

    let league = League::new(name, description);

    let mut store = MongoStore::new();
    store.create_league(league);
    println!("Created new league: \"{}\"", name);
}

/// List all leagues
fn list(_matches: &ArgMatches) {
    println!("Leagues:");
    let store = MongoStore::new();
    let leagues = store.list_leagues();
    for league in leagues {
        println!("- {} \n\tid: {}", league.name, league.id);
    }
}
