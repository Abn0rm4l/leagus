use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Some alaises to make it easier to read what ID is expected since they will
// all be Uuids. Might have to figure out a nicer way to do this later.
pub type LeagueId = Uuid;
pub type SeasonId = Uuid;

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
            id: Uuid::new_v4(),
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
}

/// A session of a season.
///
/// Each season can include one or more sessions. Sessions can be thought of
/// like "match days".
pub struct Session {
    // date: Date,
    // rounds: Vec<Round>,
}

pub struct Round {
    // matches: Vec<Match>,
    // participants: Vec<Participant>,
}

pub struct Match {
    // venue: String,
    // participants: Vec<Participant>,
}

#[derive(Debug, PartialEq)]
pub struct Participant {
    name: String,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn league_eqls() {
        let uuid = Uuid::new_v4();

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
