use serde::{Deserialize, Serialize};

use super::{LeagueId, SeasonId};

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
    pub active_season: Option<SeasonId>,
}

impl League {
    /// Creates a new [`League`].
    pub fn new(name: &str, description: &str) -> League {
        League {
            id: LeagueId::new(),
            name: String::from(name),
            description: String::from(description),
            active_season: None,
        }
    }
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
            active_season: None,
        };

        let league2 = League {
            id: uuid,
            name: String::from("Epic"),
            description: "Tim's your uncle".to_string(),
            active_season: None,
        };

        assert_eq!(league1, league2);
    }
}
