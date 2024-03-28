use bson::Uuid;
use chrono::{DateTime, Utc};
use clap::{arg, ArgMatches, Command};
use leagus::models::{SeasonId, Session};
use leagus::persistence::sync::{mongo_store::MongoStore, WriteableStore};

pub const CMD_NAME: &str = "sessions";

pub fn commands() -> Command {
    Command::new(CMD_NAME)
        .about("Commands for managing sessions")
        .subcommand_required(true)
        .subcommand(
            Command::new("create")
                .about("Create a new session")
                .arg(arg!(
                    -d --date <DATE> "Date of new sessions"
                ))
                .arg(
                    arg!(
                        -s --season <NAME> "Name of season to add the new season"
                    )
                    .required(true),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List existing sessions")
                .arg(arg!(
                    -s --season <NAME> "Filter sessions by season"
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
    let season_id = matches.get_one::<String>("season").expect("required");
    let season_id = Uuid::parse_str(season_id).expect("Invalid season id");

    // TODO: handle bad dates with more grace
    // TODO: be more flexible on date formats
    let date = matches.get_one::<String>("date");
    let date = match date {
        Some(date) => date.parse::<DateTime<Utc>>().unwrap(),
        None => Utc::now(),
    };

    let mut store = MongoStore::new();
    let season = store.get_season(&SeasonId::from(season_id));

    match season {
        Some(season) => {
            let session = Session::new(&season.id, &date);
            println!(
                "Adding new session {} to {}",
                serde_json::to_string_pretty(&session).unwrap(),
                serde_json::to_string_pretty(&season).unwrap()
            );
            store.create_session(&session);
        }
        None => println!("Cannot find league with name \"{}\".", season_id),
    }
}

/// List all leagues
fn list(_matches: &ArgMatches) {
    let store = MongoStore::new();
    let seasons = store.list_seasons();
    for season in seasons {
        println!("Season: {} ({})", season.name, season.id);
        let sessions = store.list_sessions_for_season(&season.id);
        for season in sessions {
            println!("\t- {:?}", season);
        }
    }
}
