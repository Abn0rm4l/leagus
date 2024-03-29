use serde::{Deserialize, Serialize};

use super::VenueId;

/// Somewhere to play a match. Physical or virtual!
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Venue {
    // Flatten will inline this field into its parent.
    // See https://serde.rs/attr-flatten.html
    #[serde(flatten)]
    pub id: VenueId,
    pub name: String,
}

impl Venue {
    pub fn new(name: String) -> Venue {
        Venue {
            id: VenueId::new(),
            name,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use bson::{doc, Uuid};
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_as_bson() {
        let uuid = Uuid::parse_str("00000000-2248-4345-80ec-b88499f9ff1e").unwrap();
        let id = VenueId::from(uuid);
        let name = "Center Court".to_string();

        let session = Venue {
            id,
            name: name.clone(),
        };

        let bson = bson::to_document(&session).unwrap();

        let expected_bson = doc! {
            "_id": id,
            "name": &name,
        };

        assert_eq!(bson, expected_bson);
    }
}
