// In Memory Store

use super::WriteableStore;
use crate::models::League;
use std::collections::HashMap;
use uuid::Uuid;

/// Super basic in memory store, mostly useful for testing.
pub struct InMemoryStore {
    leagues: HashMap<Uuid, League>,
}

impl InMemoryStore {
    pub fn new() -> InMemoryStore {
        InMemoryStore {
            leagues: HashMap::new(),
        }
    }
}

impl WriteableStore for InMemoryStore {
    fn create_league(&mut self, league: League) -> () {
        self.leagues.insert(league.id, league);
    }

    fn get_league(&self, league_id: &uuid::Uuid) -> Option<League> {
        match self.leagues.get(league_id) {
            Some(league) => Some(league.clone()),
            None => None,
        }
    }

    fn list_leagues(&self) -> Vec<League> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_after_create() {
        let mut store = InMemoryStore {
            leagues: HashMap::new(),
        };

        let id = Uuid::new_v4();

        let league = League {
            name: String::from("Social League"),
            id,
        };

        store.create_league(league);
        let stored_league = store.get_league(&id);

        assert_eq!(id, stored_league.unwrap().id);
    }
}
