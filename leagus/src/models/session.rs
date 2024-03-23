use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{SeasonId, SessionId};

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
