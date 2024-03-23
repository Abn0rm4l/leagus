use serde::{Deserialize, Serialize};

use super::VenueId;

/// Somewhere to play a match. Physical or virtual!
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Venue {
    #[serde(rename = "_id")]
    pub id: VenueId,
    pub name: String,
}
