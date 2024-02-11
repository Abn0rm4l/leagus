use time::Date;
use uuid::Uuid;

/// A league.
#[derive(Debug, PartialEq, Clone)]
pub struct League {
    pub name: String,
    pub id: Uuid,
    // pub participants: Vec<Participant>,
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
