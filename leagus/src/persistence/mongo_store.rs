
use mongodb::sync::{Client, Collection};

use crate::models::League;

use super::WriteableStore;

pub struct MongoStore {
    client: Client,
}

impl MongoStore {
    pub fn new() -> MongoStore {
        let client_result = Client::with_uri_str("mongodb://root:example@127.0.0.1:27017");

        match client_result {
            Ok(client) => MongoStore {
                client
            },
            Err(error) => panic!("Problem opening a connection, {:?}", error),
        }
    }
}

impl WriteableStore for MongoStore {
    fn create_league(&mut self, league: League) -> () {
        let db = self.client.database("leagus");
        let collection = db.collection::<League>("leagues");

        // TODO: Make sure we use the League.id as the mongodb id
        let _ = collection.insert_one(league, None);
    }

    fn get_league(&self, league_id: &uuid::Uuid) -> Option<League> {
        todo!()
    }

    fn list_leagues(&self) -> Vec<League> {
        // let db = self.client.database("leagus");
        // let collection = db.collection::<League>("leagues");
        let collection = league_collection(self);
        let cursor = collection.find(None, None).unwrap();

        cursor.map(|x| x.unwrap()).collect()
    }
}

impl Drop for MongoStore {
    fn drop(&mut self) {
        let client = self.client.clone();
        client.shutdown();
    }
}

// Return a handle to the MongoDB League Collection
fn league_collection(store: &MongoStore) -> Collection<League> {
    let db = store.client.database("leagus");
    db.collection::<League>("leagues")
}
