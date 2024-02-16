mod cli;

use clap::command;
use cli::{participants, leagues};

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
