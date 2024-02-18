// In Memory Store

use super::WriteableStore;
use crate::models::{League, LeagueId, Season};
use std::collections::HashMap;

/// Super basic in memory store, mostly useful for testing.
pub struct InMemoryStore {
    leagues: HashMap<LeagueId, League>,
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

    fn get_league(&self, league_id: &LeagueId) -> Option<League> {
        match self.leagues.get(league_id) {
            Some(league) => Some(league.clone()),
            None => None,
        }
    }

    fn list_leagues(&self) -> Vec<League> {
        todo!()
    }

    fn create_season(&mut self, season: &Season) -> () {
        todo!()
    }

    fn get_league_by_name(&self, league_name: &str) -> Option<League> {
        todo!()
    }

    fn list_seasons(&self) -> Vec<Season> {
        todo!()
    }

    fn list_seasons_for_league(&self, league_id: &LeagueId) -> Vec<Season> {
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

        let id = LeagueId::new();

        let league = League {
            id,
            name: String::from("Social League"),
            description: "Such fun".to_string(),
            seasons: Vec::new(),
        };

        store.create_league(league);
        let stored_league = store.get_league(&id);

        assert_eq!(id, stored_league.unwrap().id);
    }
}
