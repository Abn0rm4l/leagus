use mongodb::sync::{Client, Collection};

use crate::models::League;

use super::WriteableStore;

pub struct MongoStore {
    client: Client,
}

impl MongoStore {
    pub fn new() -> MongoStore {
        let result = Client::with_uri_str("mongodb://root:example@127.0.0.1:27017");

        match result {
            Ok(client) => MongoStore { client },
            Err(error) => panic!("Problem opening a connection, {:?}", error),
        }
    }
}

impl WriteableStore for MongoStore {
    fn create_league(&mut self, league: League) -> () {
        let collection = league_collection(self);

        // TODO: Make sure we use the League.id as the mongodb id
        let _ = collection.insert_one(league, None);
    }

    fn get_league(&self, league_id: &uuid::Uuid) -> Option<League> {
        todo!()
    }

    fn list_leagues(&self) -> Vec<League> {
        let collection = league_collection(self);
        let result = collection.find(None, None);

        match result {
            Ok(cursor) => cursor
                .filter(|x| x.is_ok()) // Errors here are probably serialization related
                .map(|x| x.unwrap())
                .collect(),
            Err(error) => {
                println!("Error finding leagues, {:?}", error);
                Vec::new()
            }
        }
    }
}

impl Drop for MongoStore {
    fn drop(&mut self) {
        // To cleanly close our connections we need to shutdown the sync client.
        let client = self.client.clone();
        client.shutdown();
    }
}

/// Return a handle to the MongoDB League Collection
fn league_collection(store: &MongoStore) -> Collection<League> {
    let db = store.client.database("leagus");
    db.collection::<League>("leagues")
}
