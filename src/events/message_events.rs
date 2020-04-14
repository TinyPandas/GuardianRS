use crate::util::*;

use serenity::{
    model::{
        channel::{Attachment, ReactionType, Message},
        id::{ChannelId, EmojiId, MessageId},
    },
    prelude::*,
};

pub struct MessageEvents;

fn add_reaction(ctx: &Context, msg: &Message, is_animated: bool, emoji_id: u64, emoji_name: &str) {
    match &msg.react(&ctx.http, ReactionType::Custom {animated: is_animated, id: EmojiId::from(emoji_id), name: Some(String::from(emoji_name))}) {
        Ok(()) => {},
        Err(why) => {
            println!("Failed to add reaction to message. {:?}", why);
        },
    }
}

pub fn message_received(ctx: Context, msg: Message) {
    let name: String = msg.channel_id.name(&ctx).unwrap();

    match name.as_str() {
        "showcase" => {
            //allow specific links 

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

pub fn message_deleted(ctx: &Context, channel_id: ChannelId, msg_id: MessageId) {
    //TODO Link with DB to get old content.

    //let msg_obj = get_message_from_channel_by_id(&ctx, channel_id, msg_id);
    //if msg_obj.is_none() { return }
    //let msg_obj = msg_obj.unwrap();

    //let author = &msg_obj.author.name;
    //let content = &msg_obj.content;

    //let name: String = channel_id.name(&ctx).unwrap();

    //let guild = get_guild_from_message(&msg_obj);
    //if guild.is_none() { return }
    //let guild = guild.unwrap();

    //let chat_log = get_channel_from_guild_by_name(&ctx, guild, String::from("chat-log"));
    //if chat_log.is_none() { return }
    //let chat_log = chat_log.unwrap();

    //let _ = chat_log.say(&ctx, &format!("{} deleted message in {} with content `{}`.", author, name, content));
}

pub fn message_edited(ctx: Context, old_cont: Option<Message>, new_cont: Option<Message>) {
    
}