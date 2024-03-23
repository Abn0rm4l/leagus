use serde::{Deserialize, Serialize};

use super::{MatchId, ParticipantId, RoundId, VenueId};

/// The main event, a match!!
///
/// This is where participants will fight it out for a result. A match will
/// involve participants and located at a venue. In the end a match will produce
/// a result.
///
/// TODO: This might need to become an Enum because there are different type of
/// matches which can be played and each may have different ways of calculate a
/// result.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Match {
    #[serde(rename = "_id")]
    pub id: MatchId,
    pub round_id: RoundId,
    pub venue_id: VenueId,
    pub participants: Vec<ParticipantId>,
    // TODO: Result/Score
}

impl Match {
    pub fn new(round_id: RoundId, venue_id: VenueId) -> Match {
        Match {
            id: MatchId::new(),
            round_id,
            venue_id,
            participants: Vec::new(),
        }
    }
}
