mod events;

pub mod commands;
pub mod util;

use log::error;
use serenity::{
    framework::standard::StandardFramework,
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

use commands::command_util::*;
use util::ShardManagerContainer;
use events::*;

fn main() {
    // Attempts to load data from .env file
    kankyo::load().expect("Failed to load .env file");

    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment.");

    let mut client = Client::new(&token, Handler).expect("Error creating client.");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix(";"))
            .help(&MY_HELP)
            .group(&GENERAL_GROUP)
            .group(&ADMIN_GROUP)
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}