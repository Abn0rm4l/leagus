use bson::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Some aliases to make it easier to read what ID is expected since they will
// all be Uuids. Might have to figure out a better way to do this later.
pub type LeagueId = Uuid;
pub type SeasonId = Uuid;
pub type SessionId = Uuid;
pub type RoundId = Uuid;
pub type MatchId = Uuid;
pub type ParticipantId = Uuid;
pub type VenueId = Uuid;

/// A league.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct League {
    // The mongodb id field is named "_id"
    #[serde(rename = "_id")]
    pub id: LeagueId,

    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub seasons: Vec<SeasonId>,
}

impl League {
    /// Creates a new [`League`].
    pub fn new(name: &str, description: &str) -> League {
        League {
            id: LeagueId::new(),
            name: String::from(name),
            description: String::from(description),
            seasons: Vec::new(),
        }
    }
}

/// A season of a league.
///
/// A season represents the scoring periods of a league.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Season {
    #[serde(rename = "_id")]
    pub id: SeasonId,
    pub league_id: LeagueId,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,

    #[serde(default)]
    pub name: Option<String>,
    // TODO: add table
    // TODO: add scoring system
    // TODO: add participants (pool of players available for the season)
}

impl Season {
    /// Create a new [`Season`] with a generated id.
    pub fn new(
        league: &LeagueId,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
        name: Option<String>,
    ) -> Season {
        // TODO check start date is before end date.
        Season {
            id: SeasonId::new(),
            league_id: *league,
            start: *start,
            end: *end,
            name,
        }
    }
}

/// A session of a season.
///
/// Each season can include one or more sessions. Sessions can be thought of
/// like "match days".
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "_id")]
    pub id: SessionId,
    pub season_id: SeasonId,
    pub date: DateTime<Utc>,
}

impl Session {
    pub fn new(season_id: &SeasonId, date: &DateTime<Utc>) -> Session {
        Session {
            id: SessionId::new(),
            season_id: *season_id,
            date: *date,
        }
    }
}

/// A round of a session.
///
/// Every session is made up of one or more rounds. Rounds will have a
/// collection of matches and often correlate to time. E.g. the 10:00 Round and
/// the 11:30 Round. Each round will have a list of participants which are
/// available to participate in matches.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Round {
    #[serde(rename = "_id")]
    pub id: RoundId,
    pub session_id: SessionId,
    pub participants: Vec<ParticipantId>,
    // TODO: add match matching strategy
}

impl Round {
    /// Create a new round with no participants (yet).
    pub fn new(session_id: SeasonId) -> Round {
        Round {
            id: RoundId::new(),
            session_id,
            participants: Vec::new(),
        }
    }
}

/// The main event, a match!!
///
/// This is where participants will fight it out for a result. A match will
/// involve participants and located at a venue. In the end a match will produce
/// a result.
///
/// TODO: This might need to become an Enum because there are different type of
/// matches which can be played and each may have different ways of calculate a
/// result.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Match {
    #[serde(rename = "_id")]
    pub id: MatchId,
    pub round_id: RoundId,
    pub venue_id: VenueId,
    pub participants: Vec<ParticipantId>,
    // TODO: Result/Score
}

impl Match {
    pub fn new(round_id: RoundId, venue_id: VenueId) -> Match {
        Match {
            id: MatchId::new(),
            round_id,
            venue_id,
            participants: Vec::new(),
        }
    }
}

/// A participant capable of participating in matches. This could be an
/// individual or team.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "_id")]
    pub id: ParticipantId,
    pub name: String,
}

/// Somewhere to play a match. Physical or virtual!
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Venue {
    #[serde(rename = "_id")]
    pub id: VenueId,
    pub name: String,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn league_eqls() {
        let uuid = LeagueId::new();

        let league1 = League {
            id: uuid,
            name: String::from("Epic"),
            description: "Tim's your uncle".to_string(),
            seasons: Vec::new(),
        };

        let league2 = League {
            id: uuid,
            name: String::from("Epic"),
            description: "Tim's your uncle".to_string(),
            seasons: Vec::new(),
        };

        assert_eq!(league1, league2);
    }
}
