use serde::{Deserialize, Serialize};
use serde_with::with_prefix;

use super::{ParticipantId, RoundId, SessionId};

// Prefix the various IDs to avoid clashes
with_prefix!(prefix_session "session");

/// A round of a session.
///
/// Every session is made up of one or more rounds. Rounds will have a
/// collection of matches and often correlate to time. E.g. the 10:00 Round and
/// the 11:30 Round. Each round will have a list of participants which are
/// available to participate in matches.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Round {
    // Flatten will inline this field into its parent.
    // See https://serde.rs/attr-flatten.html
    // The mongodb id field is named "_id"
    #[serde(flatten)]
    pub id: RoundId,
    #[serde(flatten, with = "prefix_session")]
    pub session_id: SessionId,
    pub participants: Vec<ParticipantId>,
    // TODO: add match making strategy
}

impl Round {
    /// Create a new round with no participants (yet).
    pub fn new(session_id: SessionId) -> Round {
        Round {
            id: RoundId::new(),
            session_id,
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
        let session_uuid = Uuid::parse_str("22222222-2248-4345-80ec-b88499f9ff1e").unwrap();
        let id = RoundId::from(uuid);
        let session_id = SessionId::from(session_uuid);

        let round = Round {
            id,
            session_id,
            participants: Vec::new(),
        };

        let bson = bson::to_document(&round).unwrap();

        let expected_bson = doc! {
            "_id": round.id,
            "session_id": round.session_id,
            "participants": round.participants,
        };

        assert_eq!(bson, expected_bson);
    }
}
