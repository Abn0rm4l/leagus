use serde::{Deserialize, Serialize};

use super::ParticipantId;

/// A participant capable of participating in matches. This could be an
/// individual or team.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "_id")]
    pub id: ParticipantId,
    pub name: String,
}
