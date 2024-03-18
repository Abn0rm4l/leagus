use futures::stream::StreamExt;
use mongodb::error::Result;
use mongodb::options::ClientOptions;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

use crate::models::{League, LeagueId, Match, Round, Season, SeasonId, Session, Venue};
use crate::persistence::WriteableStore;

/// Name of the MongoDB Database
const DB_NAME: &str = "leagus";

// Name of the Leagues Collection
const COLLECTION_LEAGUES: &str = "leagues";
const COLLECTION_SEASONS: &str = "seasons";
const COLLECTION_SESSIONS: &str = "sessions";
const COLLECTION_ROUNDS: &str = "rounds";
const COLLECTION_MATCHES: &str = "matches";
const COLLECTION_VENUES: &str = "venues";

#[derive(Clone)]
pub struct MongoStore {
    client: Client,
}

impl MongoStore {
    /// Create a new MongoDB-backed store.
    pub async fn new() -> Result<MongoStore> {
        let client_options =
            ClientOptions::parse_async("mongodb://root:example@127.0.0.1:27017").await?;

        let client = Client::with_options(client_options);

        match client {
            Ok(client) => Ok(MongoStore { client }),
            Err(error) => panic!("Problem opening a connection, {:?}", error),
        }
    }

    /// Bootstrap the MongoDB databases and collections
    pub async fn bootstrap(&mut self) {
        let league = self.bootstrap_leagues();
        let seasons = self.bootstrap_seasons();
        let sessions = self.bootstrap_sessions();
        let rounds = self.bootstrap_rounds();
        let matches = self.bootstrap_matches();

        // advance all asynchronously then wait for them to complete
        futures::join!(league, seasons, sessions, rounds, matches);
    }

    /// Bootstrap the leagues collection
    async fn bootstrap_leagues(&self) {
        let collection = leagues_collection(self);

        let opts = IndexOptions::builder().unique(true).build();

        // Add an index for name to easily query based on name
        let index = IndexModel::builder()
            .keys(doc! {"name": 1})
            .options(opts)
            .build();

        let _ = collection.create_index(index, None).await;
    }

    /// Bootstrap the seasons collection
    async fn bootstrap_seasons(&self) {
        let collection = seasons_collection(self);
        let index = IndexModel::builder().keys(doc! {"league_id": 1}).build();
        let _ = collection.create_index(index, None).await;
    }

    /// Bootstrap the sessions collection
    async fn bootstrap_sessions(&self) {
        let collection = sessions_collection(self);
        let index = IndexModel::builder().keys(doc! {"season_id": 1}).build();
        let _ = collection.create_index(index, None).await;
    }

    /// Bootstrap the rounds collection
    async fn bootstrap_rounds(&self) {
        let collection = round_collection(self);
        let index = IndexModel::builder().keys(doc! {"session_id": 1}).build();
        let _ = collection.create_index(index, None).await;
    }

    /// Bootstrap the matches collection
    async fn bootstrap_matches(&self) {
        let collection = match_collection(self);
        let index = IndexModel::builder().keys(doc! {"round_id": 1}).build();
        let _ = collection.create_index(index, None).await;
    }
}

impl WriteableStore for MongoStore {
    async fn create_league(&self, league: League) {
        let collection = leagues_collection(self);

        // TODO: Return some kind of error when failing to insert a document,
        // for example; when inserting a duplicate entry.
        let _ = collection.insert_one(league, None).await;
    }

    async fn create_season(&self, season: &Season, make_active: bool) {
        let seasons = seasons_collection(self);
        let _ = seasons.insert_one(season, None).await;

        if make_active {
            let leagues = leagues_collection(self);
            let _update_result = leagues
                .update_one(
                    doc! {
                        "_id": &season.league_id
                    },
                    doc! {
                        "$set": { "active_season": Some(&season.id) }
                    },
                    None,
                )
                .await;
        }
    }

    async fn create_session(&self, session: &Session) {
        let sessions = sessions_collection(self);
        let _ = sessions.insert_one(session, None).await;
    }

    async fn create_round(&self, round: &Round) {
        let rounds = round_collection(self);
        let _ = rounds.insert_one(round, None).await;
    }

    async fn create_match(&self, a_match: &Match) {
        let matches = match_collection(self);
        let _ = matches.insert_one(a_match, None).await;
    }

    async fn create_venue(&self, venue: &Venue) {
        let venues = venue_collection(self);
        let _ = venues.insert_one(venue, None).await;
    }

    async fn get_league(&self, league_id: &LeagueId) -> Option<League> {
        let leagues = leagues_collection(self);
        let result = leagues.find_one(
            doc! {
                "_id": league_id
            },
            None,
        );
        result.await.ok().unwrap_or_default()
    }

    async fn get_league_by_name(&self, league_name: &str) -> Option<League> {
        let leagues = leagues_collection(self);
        let result = leagues.find_one(
            doc! {
                "name": league_name
            },
            None,
        );
        result.await.ok().unwrap_or_default()
    }

    async fn get_season(&self, season_id: &SeasonId) -> Option<Season> {
        let seasons = seasons_collection(self);
        let result = seasons.find_one(
            doc! {
                "_id": season_id
            },
            None,
        );
        result.await.ok().unwrap_or_default()
    }

    async fn list_leagues(&self) -> Vec<League> {
        let collection = leagues_collection(self);
        let result = collection.find(None, None).await;

        match result {
            Ok(cursor) => (cursor.collect::<Vec<Result<League>>>().await)
                .into_iter()
                .filter_map(|x| x.ok())
                .collect::<Vec<League>>(),
            Err(error) => {
                println!("Error finding leagues, {:?}", error);
                Vec::new()
            }
        }
    }

    async fn list_seasons(&self) -> Vec<Season> {
        let collection = seasons_collection(self);
        let result = collection.find(None, None).await;

        match result {
            Ok(cursor) => (cursor.collect::<Vec<Result<Season>>>().await)
                .into_iter()
                .filter_map(|x| x.ok()) // TODO: log out failures
                .collect::<Vec<Season>>(),
            Err(error) => {
                println!("Error finding seasons, {:?}", error);
                Vec::new()
            }
        }
    }

    async fn list_seasons_for_league(&self, league_id: &LeagueId) -> Vec<Season> {
        let collection = seasons_collection(self);
        let result = collection
            .find(
                doc! {
                    "league_id": league_id
                },
                None,
            )
            .await;

        match result {
            Ok(cursor) => (cursor.collect::<Vec<Result<Season>>>().await)
                .into_iter()
                .filter_map(|x| x.ok()) // TODO: log out 'broken' docs
                .collect(),
            Err(error) => {
                println!(
                    "Error finding seasons for league '{:?}', {:?}",
                    league_id, error
                );
                Vec::new()
            }
        }
    }

    async fn list_sessions(&self) -> Vec<Session> {
        let collection = sessions_collection(self);
        let result = collection.find(None, None).await;

        match result {
            Ok(cursor) => (cursor.collect::<Vec<Result<Session>>>().await)
                .into_iter()
                .filter_map(|x| x.ok()) // TODO: log out 'broken' docs
                .collect(),
            Err(error) => {
                println!("Error finding sessions, {:?}", error);
                Vec::new()
            }
        }
    }

    async fn list_sessions_for_season(&self, season_id: &SeasonId) -> Vec<Session> {
        let collection = sessions_collection(self);
        let result = collection
            .find(
                doc! {
                    "season_id": season_id
                },
                None,
            )
            .await;

        match result {
            Ok(cursor) => (cursor.collect::<Vec<Result<Session>>>().await)
                .into_iter()
                .filter_map(|x| x.ok()) // TODO: log out 'broken' docs
                .collect(),
            Err(error) => {
                println!(
                    "Error finding seasons for league '{:?}', {:?}",
                    season_id, error
                );
                Vec::new()
            }
        }
    }
}

// TODO: Check if this is needed with the async client?
// impl Drop for MongoStore {
//     fn drop(&mut self) {
//         // To cleanly close our connections we need to shutdown the sync client.
//         let client = self.client.clone();
//         client.shutdown();
//     }
// }

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

/// Return a handle to the MongoDB Sessions Collection
fn sessions_collection(store: &MongoStore) -> Collection<Session> {
    let db = store.client.database(DB_NAME);
    db.collection::<Session>(COLLECTION_SESSIONS)
}

/// Return a handle to the MongoDB Rounds Collection
fn round_collection(store: &MongoStore) -> Collection<Round> {
    let db = store.client.database(DB_NAME);
    db.collection::<Round>(COLLECTION_ROUNDS)
}

/// Return a handle to the MongoDB Matches Collection
fn match_collection(store: &MongoStore) -> Collection<Match> {
    let db = store.client.database(DB_NAME);
    db.collection::<Match>(COLLECTION_MATCHES)
}

/// Return a handle to the MongoDB Matches Collection
fn venue_collection(store: &MongoStore) -> Collection<Venue> {
    let db = store.client.database(DB_NAME);
    db.collection::<Venue>(COLLECTION_VENUES)
}
