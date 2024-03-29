mod cli;

use clap::command;
use cli::{database, leagues, participants, seasons, sessions};

/// The CLI interface for Leagus
fn main() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(database::commands())
        .subcommand(leagues::commands())
        .subcommand(seasons::commands())
        .subcommand(sessions::commands())
        .subcommand(participants::commands())
        .get_matches();

    match matches.subcommand() {
        Some((database::CMD_NAME, sub_matches)) => database::handle_subcommands(sub_matches),
        Some((leagues::CMD_NAME, sub_matches)) => leagues::handle_subcommands(sub_matches),
        Some((seasons::CMD_NAME, sub_matches)) => seasons::handle_subcommands(sub_matches),
        Some((sessions::CMD_NAME, sub_matches)) => sessions::handle_subcommands(sub_matches),
        Some((participants::CMD_NAME, sub_matches)) => {
            participants::handle_subcommands(sub_matches)
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
