use mongodb::{sync::{Client, Collection}, bson::doc, options::IndexOptions, IndexModel};

use crate::models::{League, Season, LeagueId};

use super::WriteableStore;

/// Name of the MongoDB Database
const DB_NAME: &str = "leagus";

// Name of the Leagues Collection
const COLLECTION_LEAGUES: &str = "leagues";
const COLLECTION_SEASONS: &str = "seasons";

pub struct MongoStore {
    client: Client,
}

impl MongoStore {
    /// Create a new MongoDB-backed store.
    pub fn new() -> MongoStore {
        let result = Client::with_uri_str("mongodb://root:example@127.0.0.1:27017");

        match result {
            Ok(client) => MongoStore { client },
            Err(error) => panic!("Problem opening a connection, {:?}", error),
        }
    }

    /// Bootstrap the MongoDB databases and collections
    pub fn bootstrap(&mut self) {
        self.bootstrap_leagues();
        self.bootstrap_seasons();
    }

    /// Bootstrap the leagues collection
    fn bootstrap_leagues(&mut self) {
        let collection = leagues_collection(self);

        let opts = IndexOptions::builder()
            .unique(true)
            .build();

        // Add an index for name to easily query based on name
        let index = IndexModel::builder()
            .keys(doc! {"name": 1})
            .options(opts)
            .build();

        let _ = collection.create_index(index, None);
    }

    /// Bootstrap the seasons collection
    fn bootstrap_seasons(&mut self) {
        let collection = seasons_collection(self);

        let index = IndexModel::builder()
            .keys(doc! {"league_id": 1})
            .build();

        let _ = collection.create_index(index, None);
    }
}

impl WriteableStore for MongoStore {
    fn create_league(&mut self, league: League) -> () {
        let collection = leagues_collection(self);

        // TODO: Return some kind of error when failing to insert a document,
        // for example; when inserting a duplicate entry.
        let _ = collection.insert_one(league, None);
    }

    fn get_league(&self, league_id: &LeagueId) -> Option<League> {
        let leagues = leagues_collection(self);
        let result = leagues.find_one(
            doc! {
                "_id": league_id
            },
            None
        );
        result.unwrap()
    }

    fn get_league_by_name(&self, league_name: &str) -> Option<League> {
        let leagues = leagues_collection(self);
        let result = leagues.find_one(
            doc! {
                "name": league_name
            },
            None
        );
        result.unwrap()
    }

    fn list_leagues(&self) -> Vec<League> {
        let collection = leagues_collection(self);
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

    fn create_season(&mut self, season: &Season) -> () {
        let seasons = seasons_collection(self);
        let leagues = leagues_collection(self);

        // TODO: wrap this in a transaction The season should only be added if
        // it can be added to the league
        let _ = seasons.insert_one(season, None);

        // Add the seasons to the list of seasons
        let league = self.get_league(&season.league_id).unwrap();
        let mut seasons = league.seasons;
        seasons.push(season.league_id);

        let _update_result = leagues.update_one(
            doc! {
                "_id": league.id
            },
            doc! {
                "$set": { "seasons": seasons }
            },
            None
        );
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
fn leagues_collection(store: &MongoStore) -> Collection<League> {
    let db = store.client.database(DB_NAME);
    db.collection::<League>(COLLECTION_LEAGUES)
}

/// Return a handle to the MongoDB Seasons Collection
fn seasons_collection(store: &MongoStore) -> Collection<Season> {
    let db = store.client.database(DB_NAME);
    db.collection::<Season>(COLLECTION_SEASONS)
}
