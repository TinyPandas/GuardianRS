use serenity::client::bridge::gateway::ShardManager;
use std::sync::Arc;
use serenity::prelude::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}