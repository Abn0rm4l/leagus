pub mod mongo_store;
pub mod sync;

use crate::models::{
    League, LeagueId, Match, Participant, ParticipantId, Round, RoundId, Season, SeasonId, Session,
    SessionId, Venue,
};

/// Defines interactions with a write store.
///
// See https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html?highlight=async%20fn#async-fn-in-trait
#[allow(async_fn_in_trait)]
pub trait WriteableStore {
    // TODO: These should return an option in case of failure.

    /// Create a new [`League`].
    async fn create_league(&self, league: League);

    /// Create a new [`Season`].
    async fn create_season(&self, season: &Season, make_active: bool);

    /// Create a new [`Session`].
    async fn create_session(&self, session: &Session);

    /// Create a new [`Round`].
    async fn create_round(&self, round: &Round);

    /// Create a new [`Match`].
    async fn create_match(&self, a_match: &Match);

    /// Create a new [`Venue`].
    async fn create_venue(&self, venue: &Venue);

    /// Create a new [`Participant`].
    async fn create_participant(&self, participant: &Participant);

    /// Add a [`Participant`] to a [`Round`].
    async fn add_participant_to_round(&self, participant_id: &ParticipantId, round_id: &RoundId);

    /// Get the [`League`] from the store with the matching ID.
    async fn get_league(&self, league_id: &LeagueId) -> Option<League>;

    /// Get [`League`] by name.
    async fn get_league_by_name(&self, league_name: &str) -> Option<League>;

    /// Get [`Season`] by id
    async fn get_season(&self, season_id: &SeasonId) -> Option<Season>;

    /// Get [`Session`] by id
    async fn get_session(&self, season_id: &SessionId) -> Option<Session>;

    /// Get [`Round`] by id
    async fn get_round(&self, round_id: &RoundId) -> Option<Round>;

    /// Get [`Participant`] by id
    async fn get_participant(&self, participant_id: &ParticipantId) -> Option<Participant>;

    /// List all the leagues
    async fn list_leagues(&self) -> Vec<League>;

    // /// List all the seasons
    async fn list_seasons(&self) -> Vec<Season>;
    async fn list_seasons_for_league(&self, league_id: &LeagueId) -> Vec<Season>;

    async fn list_sessions(&self) -> Vec<Session>;
    async fn list_sessions_for_season(&self, season_id: &SeasonId) -> Vec<Session>;

    async fn list_rounds_for_session(&self, session_id: &SessionId) -> Vec<Round>;

    async fn list_venues(&self) -> Vec<Venue>;

    async fn list_participants(&self, query_name: Option<String>) -> Vec<Participant>;
    async fn list_participants_for_round(&self, round_id: &RoundId) -> Vec<Participant>;
}
