use mongodb::{
    Client, Database, Collection,
    results::{
        InsertOneResult
    },
    error::{
        Error
    }
};

use once_cell::sync::OnceCell;
use bson::Document;
pub struct DatabaseMod;

static MONGO: OnceCell<Client> = OnceCell::new();

fn get_client() -> &'static Client {
    MONGO.get().expect("Client is not setup.")
}

pub fn db_setup() {
    println!("Setting up DB.");

    let client = match Client::with_uri_str("mongodb://localhost:27017") {
        Ok(client) => {
            println!("Connected to DB.");
            Some(client)
        }, Err(why) => {
            println!("Failed to setup DB. {:?}", why);
            None
        }
    };

    if client.is_none() {
        //handle no client connection to db.
        println!("No DB connection");
    }

    let _ = MONGO.set(client.unwrap()).unwrap();
}

pub fn check_db() {
    let c = get_client();

    let db = c.database("shared");

    for cols in db.list_collection_names(None) {
        for col_name in cols {
            println!("Collection: {}", col_name);
        }
    }
}

pub fn get_database(name: &str) -> Database {
    let cl = get_client();

    cl.database(name)
}

pub fn get_collection(db_name: &str, col_name: &str) -> Collection {
    let db = get_database(db_name);

    db.collection(col_name)
}

pub fn add_document(col: Collection, doc: Document) -> Result<InsertOneResult, Error> {
    col.insert_one(doc, None)
}