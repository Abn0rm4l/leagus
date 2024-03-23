mod a_match;
mod id;
mod league;
mod participant;
mod points_table;
mod round;
mod season;
mod session;
mod venue;

// Rexport sub-modules into this module for ease of use.
pub use self::a_match::*;
pub use self::id::*;
pub use self::league::*;
pub use self::participant::*;
pub use self::points_table::*;
pub use self::round::*;
pub use self::season::*;
pub use self::session::*;
pub use self::venue::*;

// Aliases for typesafe IDs
pub type LeagueId = ID<League>;
pub type SeasonId = ID<Season>;
pub type SessionId = ID<Session>;
pub type RoundId = ID<Round>;
pub type MatchId = ID<Match>;
pub type ParticipantId = ID<Participant>;
pub type VenueId = ID<Venue>;
