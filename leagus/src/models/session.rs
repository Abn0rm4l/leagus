use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::with_prefix;

use super::{SeasonId, SessionId};

with_prefix!(prefix_season "season");

/// A session of a season.
///
/// Each season can include one or more sessions. Sessions can be thought of
/// like "match days".
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Session {
    // Flatten will inline this field into its parent.
    // See https://serde.rs/attr-flatten.html
    // The mongodb id field is named "_id"
    #[serde(flatten)]
    pub id: SessionId,
    #[serde(flatten, with = "prefix_season")]
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use bson::{doc, Uuid};
    use chrono::TimeZone;
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_as_bson() {
        let season_uuid = Uuid::parse_str("00000000-2248-4345-80ec-b88499f9ff1e").unwrap();
        let session_uuid = Uuid::parse_str("11111111-2248-4345-80ec-b88499f9ff1e").unwrap();
        let season_id = SeasonId::from(season_uuid);
        let session_id = SessionId::from(session_uuid);
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 00, 00).unwrap();

        let session = Session {
            id: session_id,
            season_id,
            date: dt,
        };

        let bson = bson::to_document(&session).unwrap();

        let expected_bson = doc! {
            "_id": session_id,
            "season_id": season_id,
            "date": "2024-01-01T12:00:00Z",
        };

        assert_eq!(bson, expected_bson);
    }
}
