use clap::{arg, ArgMatches, Command};
use leagus::persistence::sync::{mongo_store::MongoStore, WriteableStore};

pub const CMD_NAME: &str = "database";

pub fn commands() -> Command {
    Command::new(CMD_NAME)
        .about("Commands for managing the Leagus database")
        .subcommand_required(true)
        .subcommand(Command::new("bootstrap").about("Bootstrap the database"))
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
        Some(("bootstrap", sub_matches)) => bootstrap(sub_matches),
        Some(("list", sub_matches)) => list(sub_matches),
        _ => unreachable!("Must specify a subcommand"),
    }
}

fn bootstrap(_matches: &ArgMatches) {
    let mut store = MongoStore::new();
    store.bootstrap();
    println!("Bootstrapped the Leagus database");
}

fn list(_matches: &ArgMatches) {
    println!("Leagues:");
    let store = MongoStore::new();
    let leagues = store.list_leagues();
    for league in leagues {
        println!("- {} \n\tid: {}", league.name, league.id);
    }
}
