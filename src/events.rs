use log::info;
use serenity::{
    model::{
        channel::{Attachment, ReactionType, Message},
        event::ResumedEvent,
        gateway::Ready,
        id::EmojiId,
    },
    prelude::*,
};

pub struct Handler;

fn add_reaction(ctx: &Context, msg: &Message, is_animated: bool, emoji_id: u64, emoji_name: &str) {
    match &msg.react(&ctx.http, ReactionType::Custom {animated: is_animated, id: EmojiId::from(emoji_id), name: Some(String::from(emoji_name))}) {
        Ok(()) => {},
        Err(why) => {
            println!("Failed to add reaction to message. {:?}", why);
        },
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let name: String = msg.channel_id.name(&ctx).unwrap();

        match name.as_str() {
            "showcase" => {
                let items: Vec<Attachment> = msg.attachments.clone();

                if items.len() == 0 {
                    match msg.delete(ctx.http) {
                        Ok(()) => {},
                        Err(why) => {
                            println!("Missing permission to delete messages. {:?}", why);
                        },
                    }
                };
            },
            "suggestion-box" => {
                if msg.content.starts_with("--sug--") {
                    add_reaction(&ctx, &msg, false, 693278870681419867, "upvote");
                    add_reaction(&ctx, &msg, false, 693278893121208391, "downvote");
                }
            },
            _ => {}
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}