use time::Date;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// A league.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct League {
    // The mongodb id field is named "_id"
    #[serde(rename = "_id")]
    pub id: Uuid,

    pub name: String,

    #[serde(default)]
    pub description: String,
}

impl League {
    /// Creates a new [`League`].
    pub fn new(name: &str, description: &str) -> League {
        League {
            id: Uuid::new_v4(),
            name: String::from(name),
            description: String::from(description),
        }
    }
}

/// A season of a league.
///
/// A season represents the scoring periods of a league.
pub struct Season {
    start: Date,
    end: Date,
    sessions: Vec<Session>,
}

/// A session of a season.
///
/// Each season can include one or more sessions. Sessions can be thought of
/// like "match days".
pub struct Session {
    date: Date,
    rounds: Vec<Round>,
}

pub struct Round {
    matches: Vec<Match>,
    participants: Vec<Participant>,
}

pub struct Match {
    venue: String,
    participants: Vec<Participant>,
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
        let participant = Participant {
            name: String::from("Bob"),
        };
        let participants = vec![participant];
        let uuid = Uuid::new_v4();

        let league1 = League {
            name: String::from("Epic"),
            id: uuid,
            // participants,
        };

        let participant = Participant {
            name: String::from("Bob"),
        };
        let participants = vec![participant];

        let league2 = League {
            name: String::from("Epic"),
            id: uuid,
            // participants,
        };

        assert_eq!(league1, league2);
    }
}
