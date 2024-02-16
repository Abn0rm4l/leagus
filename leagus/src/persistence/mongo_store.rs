use mongodb::{sync::{Client, Collection}, bson::doc, options::IndexOptions, IndexModel};

use crate::models::League;

use super::WriteableStore;

/// Name of the MongoDB Database
const DB_NAME: &str = "leagus";

// Name of the Leagues Collection
const COLLECTION_LEAGUES: &str = "leagues";

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

        // Make sure the name is unique
        // TODO: Move this into a db setup/bootstrap function
        let opts = IndexOptions::builder()
            .unique(true)
            .build();

        let index = IndexModel::builder()
            .keys(doc! {"name": -1})
            .options(opts)
            .build();

        let _ = collection.create_index(index, None);
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
                .map(|x| x.unwrap())   // TODO: log out 'broken' docs
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
    let db = store.client.database(DB_NAME);
    db.collection::<League>(COLLECTION_LEAGUES)
}
