pub mod in_memory_store;
pub mod mongo_store;

use crate::models::{League, Season, LeagueId};

/// Defines interactions with a write store.
pub trait WriteableStore {
    // TODO: These should return an option in case of failure.

    /// Create a new League.
    fn create_league(&mut self, league: League) -> ();

    /// Get the League from the store with the matching ID.
    fn get_league(&self, league_id: &LeagueId) -> Option<League>;

    /// Get [`League`] by name.
    fn get_league_by_name(&self, league_name: &str) -> Option<League>;

    /// List all the leagues
    fn list_leagues(&self) -> Vec<League>;

    /// List all the seasons
    fn list_seasons(&self) -> Vec<Season>;

    /// List all the seasons for the league
    fn list_seasons_for_league(&self, league_id: &LeagueId) -> Vec<Season>;

    /// Create a new league season
    fn create_season(&mut self, season: &Season) -> ();
}
