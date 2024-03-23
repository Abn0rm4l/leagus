use bson::Uuid;
use serde::{Deserialize, Serialize};

use super::{ParticipantId, RoundId, SessionId};

/// A round of a session.
///
/// Every session is made up of one or more rounds. Rounds will have a
/// collection of matches and often correlate to time. E.g. the 10:00 Round and
/// the 11:30 Round. Each round will have a list of participants which are
/// available to participate in matches.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Round {
    #[serde(rename = "_id")]
    pub id: RoundId,
    pub session_id: SessionId,
    pub participants: Vec<ParticipantId>,
    // TODO: add match matching strategy
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
