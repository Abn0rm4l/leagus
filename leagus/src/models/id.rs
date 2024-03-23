use std::fmt::Display;

use bson::Bson;
use bson::Uuid;
use serde::{Deserialize, Serialize};

/// Generic container for the various IDs.
/// Each type of ID is really just a Uuid but is a distinct type.
/// E.g. an ID<League> is a different type from ID<Season>.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ID<T> {
    // Flatten will inline this field into its parent.
    // See https://serde.rs/attr-flatten.html
    #[serde(rename = "_id", flatten)]
    pub id: Uuid,

    #[serde(skip)]
    id_type: std::marker::PhantomData<T>,
}

impl<T> ID<T> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new(),
            id_type: std::marker::PhantomData,
        }
    }
}

impl<T> From<ID<T>> for Bson {
    fn from(val: ID<T>) -> Self {
        Bson::from(val.id)
    }
}

impl<T> From<Uuid> for ID<T> {
    fn from(value: Uuid) -> Self {
        Self {
            id: value,
            id_type: std::marker::PhantomData,
        }
    }
}

// A custom Clone impl is required because the derive version requies the type paramter to also
// implement Clone which we don't need or want here.
impl<T> Clone for ID<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            id_type: self.id_type,
        }
    }
}

impl<T> Copy for ID<T> {}

impl<T> Display for ID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}
