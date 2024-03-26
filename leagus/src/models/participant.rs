use serde::{Deserialize, Serialize};

use super::ParticipantId;

/// A participant capable of participating in matches. This could be an
/// individual or team.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Participant {
    // Flatten will inline this field into its parent.
    // See https://serde.rs/attr-flatten.html
    #[serde(flatten)]
    pub id: ParticipantId,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::{doc, Uuid};
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_as_bson() {
        let uuid = Uuid::parse_str("00000000-2248-4345-80ec-b88499f9ff1e").unwrap();
        let id = ParticipantId::from(uuid);
        let name = "Jacob".to_string();

        let participant = Participant { id, name };

        let bson = bson::to_document(&participant).unwrap();

        let expected_bson = doc! {
            "_id": participant.id,
            "name": participant.name,
        };

        assert_eq!(bson, expected_bson);
    }
}
