use chrono::{DateTime, Utc, TimeDelta};
use clap::{arg, ArgMatches, Command};
use leagus::models::{League, Season};
use leagus::persistence::mongo_store::MongoStore;
use leagus::persistence::WriteableStore;

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
                ))
        )
        .subcommand(
            Command::new("add-season")
                .about("Add a new season")
                .arg(arg!(
                    -s --start <DATE> "Start date of new season"
                ))
                .arg(arg!(
                    -e --end <DATE> "End date of new season"
                ))
                .arg(
                    arg!(
                        -l --league <NAME> "Name of league to add the new season"
                    )
                    .required(true)
                )
        )
}

pub fn handle_subcommands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("create", sub_matches)) => create(sub_matches),
        Some(("list", sub_matches)) => list(sub_matches),
        Some(("add-season", sub_matches)) => add_season(sub_matches),
        _ => unreachable!("Must specify a subcommand"),
    }
}

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

fn list(_matches: &ArgMatches) {
    println!("Leagues:");
    let store = MongoStore::new();
    let leagues = store.list_leagues();
    for league in leagues {
        println!("- {} \n\tid: {}", league.name, league.id);
    }
}

fn add_season(matches: &ArgMatches) {
    let league_name = matches.get_one::<String>("league").expect("required");
    let mut store = MongoStore::new();
    let league = store.get_league_by_name(&league_name);

    match league {
        Some(league) => {
            println!("Adding new season to {:?}", league);

            // TODO: Use the dates if supplied
            let season = Season::new(
                &league.id,
                &Utc::now(),
                &(Utc::now() + TimeDelta::days(30))
            );

            store.create_season(&season);
        },
        None => println!("Cannot find league with name \"{}\".", league_name),
    }

}
