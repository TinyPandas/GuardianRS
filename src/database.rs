use mongodb::{error::Error, results::InsertOneResult, Client, Collection, Database};

use serenity::model::id::{ChannelId, MessageId};

use bson::*;
use once_cell::sync::OnceCell;
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
        }
        Err(why) => {
            println!("Failed to setup DB. {:?}", why);
            None
        }
    };

    if client.is_none() {
        //handle no client connection to db.
        println!("No DB connection");
    }

    MONGO.set(client.unwrap()).unwrap();
}

pub fn check_db() {
    let c = get_client();

    let db = c.database("shared");

    if let Ok(cols) = db.list_collection_names(None) {
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

pub fn get_document_from_collection(col: Collection, filter: Document) -> Option<Document> {
    match col.find_one(filter, None) {
        Ok(opt_doc) => opt_doc,
        Err(_why) => None,
    }
}

pub fn add_document(col: Collection, doc: Document) -> Result<InsertOneResult, Error> {
    println!("Adding document to {}", col.name());
    col.insert_one(doc, None)
}

pub fn create_document_chat_delete(msg_id: MessageId, u_id: &str, content: &str) -> Document {
    doc! {
        "messageID": msg_id.to_string(),
        "authorID": u_id,
        "before": content
    }
}

pub fn create_document_chat_log(
    msg_id: MessageId,
    u_id: &str,
    c_id: ChannelId,
    content: &str,
) -> Document {
    doc! {
        "messageID": msg_id.to_string(),
        "authorID": u_id,
        "channelID": c_id.to_string(),
        "original": content,
        "current": content,
        "edited": false,
        "deleted": false
    }
}
