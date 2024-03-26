use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::with_prefix;

use super::{LeagueId, PointsTable, SeasonId, SessionId};

// Prefix the league _id with league
with_prefix!(prefix_league "league");
with_prefix!(prefix_active_session "active_session");

/// A season of a league.
///
/// A season represents the scoring periods of a league.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Season {
    // Flatten will inline this field into its parent.
    // See https://serde.rs/attr-flatten.html
    #[serde(flatten)]
    pub id: SeasonId,
    #[serde(flatten, with = "prefix_league")]
    pub league_id: LeagueId,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    #[serde(default)]
    pub name: String,
    pub table: PointsTable,
    #[serde(default, flatten, with = "prefix_active_session")]
    pub active_session: Option<SessionId>,
    // TODO: add scoring system
    // TODO: add participants (pool of players available for the season)?
}

impl Season {
    /// Create a new [`Season`] with a generated id.
    pub fn new(
        league: &LeagueId,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
        name: &str,
    ) -> Season {
        // TODO check start date is before end date.
        Season {
            id: SeasonId::new(),
            league_id: *league,
            start: *start,
            end: *end,
            name: name.to_string(),
            table: PointsTable {
                entries: Vec::new(),
            },
            active_session: None,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use bson::{doc, Uuid};
    use chrono::TimeZone;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use serde_json::json;

    #[test]
    fn serialize_as_json_no_active_session() {
        let season_uuid = Uuid::parse_str("00000000-2248-4345-80ec-b88499f9ff1e").unwrap();
        let season_id = SeasonId::from(season_uuid);
        let league_uuid = Uuid::parse_str("a8d1d978-2248-4345-80ec-b88499f9ff1e").unwrap();
        let league_id = LeagueId::from(league_uuid);
        let name = String::from("Test Season");
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 00, 00).unwrap();

        let season = Season {
            id: season_id,
            league_id,
            name,
            start: dt,
            end: dt,
            table: PointsTable::new(),
            active_session: None,
        };

        let json = serde_json::to_string(&season).unwrap();

        let expected_json = json!({
            "_id": "00000000-2248-4345-80ec-b88499f9ff1e",
            "league_id": "a8d1d978-2248-4345-80ec-b88499f9ff1e",
            "start": "2024-01-01T12:00:00Z",
            "end": "2024-01-01T12:00:00Z",
            "name": "Test Season",
            "table": { "entries":[] }
        });

        assert_str_eq!(json, expected_json.to_string());
    }

    #[test]
    fn serialize_as_bson_no_active_session() {
        let season_uuid = Uuid::parse_str("00000000-2248-4345-80ec-b88499f9ff1e").unwrap();
        let season_id = SeasonId::from(season_uuid);
        let league_uuid = Uuid::parse_str("a8d1d978-2248-4345-80ec-b88499f9ff1e").unwrap();
        let league_id = LeagueId::from(league_uuid);
        let name = String::from("Test Season");
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 00, 00).unwrap();

        let season = Season {
            id: season_id,
            league_id,
            name,
            start: dt,
            end: dt,
            table: PointsTable::new(),
            active_session: None,
        };

        let bson = bson::to_document(&season).unwrap();

        let expected_bson = doc! {
            "_id": season_id,
            "league_id": league_id,
            "start": "2024-01-01T12:00:00Z",
            "end": "2024-01-01T12:00:00Z",
            "name": "Test Season",
            "table": { "entries":[] }
        };

        assert_eq!(bson, expected_bson);
    }

    #[test]
    fn serialize_as_bson_with_active_session() {
        let season_uuid = Uuid::parse_str("00000000-2248-4345-80ec-b88499f9ff1e").unwrap();
        let league_uuid = Uuid::parse_str("a8d1d978-2248-4345-80ec-b88499f9ff1e").unwrap();
        let session_uuid = Uuid::parse_str("11111111-2248-4345-80ec-b88499f9ff1e").unwrap();
        let season_id = SeasonId::from(season_uuid);
        let league_id = LeagueId::from(league_uuid);
        let session_id = SessionId::from(session_uuid);
        let name = String::from("Test Season");
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 00, 00).unwrap();

        let season = Season {
            id: season_id,
            league_id,
            name,
            start: dt,
            end: dt,
            table: PointsTable::new(),
            active_session: Some(session_id),
        };

        let bson = bson::to_document(&season).unwrap();

        let expected_bson = doc! {
            "_id": season_id,
            "league_id": league_id,
            "start": "2024-01-01T12:00:00Z",
            "end": "2024-01-01T12:00:00Z",
            "name": "Test Season",
            "table": { "entries":[] },
            "active_session_id": session_id,
        };

        assert_eq!(bson, expected_bson);
    }
}
