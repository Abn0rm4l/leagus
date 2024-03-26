use serde::{Deserialize, Serialize};
use serde_with::with_prefix;

use super::{MatchId, ParticipantId, RoundId, VenueId};

// Prefix the various IDs to avoid clashes
with_prefix!(prefix_round "round");
with_prefix!(prefix_venue "venue");

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
    // Flatten will inline this field into its parent.
    // See https://serde.rs/attr-flatten.html
    #[serde(flatten)]
    pub id: MatchId,
    #[serde(flatten, with = "prefix_round")]
    pub round_id: RoundId,
    #[serde(flatten, with = "prefix_venue")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use bson::{doc, Uuid};
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_as_bson() {
        let uuid = Uuid::parse_str("00000000-2248-4345-80ec-b88499f9ff1e").unwrap();
        let round_uuid = Uuid::parse_str("11111111-2248-4345-80ec-b88499f9ff1e").unwrap();
        let venue_uuid = Uuid::parse_str("22222222-2248-4345-80ec-b88499f9ff1e").unwrap();
        let id = MatchId::from(uuid);
        let round_id = RoundId::from(round_uuid);
        let venue_id = VenueId::from(venue_uuid);

        let participant = Match {
            id,
            round_id,
            venue_id,
            participants: Vec::new(),
        };

        let bson = bson::to_document(&participant).unwrap();

        let expected_bson = doc! {
            "_id": participant.id,
            "round_id": participant.round_id,
            "venue_id": participant.venue_id,
            "participants": participant.participants,
        };

        assert_eq!(bson, expected_bson);
    }
}
