use serde::{Deserialize, Serialize};

use super::ParticipantId;

/// A points table for the season
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointsTable {
    #[serde(default)]
    pub entries: Vec<PointsTableEntry>,
}

impl PointsTable {
    pub fn new() -> PointsTable {
        PointsTable {
            entries: Vec::new(),
        }
    }
}

/// A single entry in the scoring table
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointsTableEntry {
    pub participant_name: String,
    pub participant_id: ParticipantId,
    pub points: u32,
    pub wins: u32,
    pub losses: u32,
}
