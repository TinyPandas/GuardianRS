use crate::events::message_events::*;
use crate::database::*;

use log::info;
use serenity::{
    model::{
        channel::Message,
        event::{MessageUpdateEvent, ResumedEvent},
        gateway::Ready,
        id::{ChannelId, MessageId},
    },
    prelude::*,
};

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) { message_received(ctx, msg); }
    fn message_delete(&self, ctx: Context, channel_id: ChannelId, msg_id: MessageId) { message_deleted(&ctx, channel_id, msg_id); }
    fn message_update(&self, ctx: Context, old_cont: Option<Message>, new_cont: Option<Message>, _event: MessageUpdateEvent) { message_edited(ctx, old_cont, new_cont); }

    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    
        //init db
        println!("Setting up DB.");
        db_setup();
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}