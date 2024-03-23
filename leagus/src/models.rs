use bson::Uuid;

mod a_match;
mod league;
mod participant;
mod points_table;
mod round;
mod season;
mod session;
mod venue;

// Rexport sub-modules into this module for ease of use.
pub use self::a_match::*;
pub use self::league::*;
pub use self::participant::*;
pub use self::points_table::*;
pub use self::round::*;
pub use self::season::*;
pub use self::session::*;
pub use self::venue::*;

// Some aliases to make it easier to read what ID is expected since they will
// all be Uuids. Might have to figure out a better way to do this later.
pub type LeagueId = Uuid;
pub type SeasonId = Uuid;
pub type SessionId = Uuid;
pub type RoundId = Uuid;
pub type MatchId = Uuid;
pub type ParticipantId = Uuid;
pub type VenueId = Uuid;
