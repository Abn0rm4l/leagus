use chrono::{DateTime, TimeDelta, Utc};
use clap::{arg, ArgMatches, Command};
use leagus::models::Season;
use leagus::persistence::mongo_store::MongoStore;
use leagus::persistence::WriteableStore;

pub const CMD_NAME: &str = "seasons";

pub fn commands() -> Command {
    Command::new(CMD_NAME)
        .about("Commands for managing seasons")
        .subcommand_required(true)
        .subcommand(
            Command::new("create")
                .about("Create a new season")
                .arg(arg!(
                    -s --start <DATE> "Start date of new season"
                ))
                .arg(arg!(
                    -e --end <DATE> "End date of new season"
                ))
                .arg(arg!(
                    -n --name <NAME> "Name of the new season"
                ))
                .arg(
                    arg!(
                        -l --league <NAME> "Name of league to add the new season"
                    )
                    .required(true),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List existing seasons")
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

/// Add a new season to a league
fn create(matches: &ArgMatches) {
    let league_name = matches.get_one::<String>("league").expect("required");

    // TODO: handle bad dates with more grace
    // TODO: be more flexible on date formats
    let start = matches.get_one::<String>("start");
    let start = match start {
        Some(start) => start.parse::<DateTime<Utc>>().unwrap(),
        None => Utc::now(),
    };

    let end = matches.get_one::<String>("end");
    let end = match end {
        Some(end) => end.parse::<DateTime<Utc>>().unwrap(),
        None => start + TimeDelta::days(30),
    };

    let name = matches.get_one::<String>("name");

    let mut store = MongoStore::new();
    let league = store.get_league_by_name(league_name);

    match league {
        Some(league) => {
            println!("Adding new season to {:?}", league);

            let season = Season::new(&league.id, &start, &end, name.cloned());

            store.create_season(&season);
        }
        None => println!("Cannot find league with name \"{}\".", league_name),
    }
}

/// List all leagues
fn list(_matches: &ArgMatches) {
    let store = MongoStore::new();
    let leagues = store.list_leagues();
    for league in leagues {
        println!("League: {}", league.name);
        let seasons = store.list_seasons_for_league(&league.id);
        for season in seasons {
            println!("\t- {:?}", season);
        }
    }
}
