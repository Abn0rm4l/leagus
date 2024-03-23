use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{LeagueId, PointsTable, SeasonId};

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
    pub name: String,

    pub table: PointsTable,
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
        }
    }
}
