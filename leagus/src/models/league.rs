use serde::{Deserialize, Serialize};
use serde_with::with_prefix;

use super::{LeagueId, SeasonId};

// Prefix the various IDs to avoid clashes
with_prefix!(prefix_active_season "active_season");

/// A league.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct League {
    // Flatten will inline the ID's UUID field into this struct.
    // See https://serde.rs/attr-flatten.html
    #[serde(flatten)]
    pub id: LeagueId,

    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default, flatten, with = "prefix_active_season")]
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
    use bson::{doc, Uuid};
    use pretty_assertions::{assert_eq, assert_str_eq};
    use serde_json::json;

    #[test]
    fn serialize_as_json_no_active_season() {
        let uuid = Uuid::parse_str("a8d1d978-2248-4345-80ec-b88499f9ff1e").unwrap();
        let id = LeagueId::from(uuid);
        let name = String::from("Epic");
        let description = "Tim's your uncle".to_string();

        let league1 = League {
            id,
            name,
            description,
            active_season: None,
        };

        let json = serde_json::to_string(&league1).unwrap();

        let expected_json = json!({
            "_id":"a8d1d978-2248-4345-80ec-b88499f9ff1e",
            "name":"Epic",
            "description":"Tim's your uncle",
        });

        assert_str_eq!(json, expected_json.to_string());
    }

    #[test]
    fn serialize_as_bson_no_active_season() {
        let uuid = Uuid::parse_str("a8d1d978-2248-4345-80ec-b88499f9ff1e").unwrap();
        let id = LeagueId::from(uuid);
        let name = String::from("Epic");
        let description = "Tim's your uncle".to_string();

        let league = League {
            id,
            name,
            description,
            active_season: None,
        };

        let bson = bson::to_document(&league).unwrap();

        let expected_bson = doc! {
            "_id": uuid,
            "name": "Epic",
            "description": "Tim's your uncle",
        };

        assert_eq!(bson, expected_bson);
    }

    #[test]
    fn serialize_as_bson_with_active_season() {
        let uuid = Uuid::parse_str("a8d1d978-2248-4345-80ec-b88499f9ff1e").unwrap();
        let id = LeagueId::from(uuid);
        let name = String::from("Epic");
        let description = "Tim's your uncle".to_string();

        let uuid_season = Uuid::parse_str("a8d1d978-2248-4345-80ec-b88499f9ff1e").unwrap();
        let season_id = SeasonId::from(uuid_season);

        let league = League {
            id,
            name,
            description,
            active_season: Some(season_id),
        };

        let bson = bson::to_document(&league).unwrap();

        let expected_bson = doc! {
            "_id": uuid,
            "name": "Epic",
            "description": "Tim's your uncle",
            "active_season_id": season_id
        };

        assert_eq!(bson, expected_bson);
    }
}
