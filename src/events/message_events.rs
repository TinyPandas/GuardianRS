use crate::database::*;
use crate::util::*;

use bson::*;
use serenity::{
    model::{
        channel::{Attachment, Message, ReactionType},
        id::{ChannelId, EmojiId, MessageId, UserId},
    },
    prelude::*,
};

pub struct MessageEvents;

fn add_reaction(ctx: &Context, msg: &Message, is_animated: bool, emoji_id: u64, emoji_name: &str) {
    match &msg.react(
        &ctx.http,
        ReactionType::Custom {
            animated: is_animated,
            id: EmojiId::from(emoji_id),
            name: Some(String::from(emoji_name)),
        },
    ) {
        Ok(()) => {}
        Err(why) => {
            println!("Failed to add reaction to message. {:?}", why);
        }
    }
}

pub fn message_received(ctx: Context, msg: Message) {
    let name: String = msg.channel_id.name(&ctx).unwrap();

    let doc = create_document_chat_log(
        msg.id,
        &msg.author.id.to_string(),
        msg.channel_id,
        &msg.content,
    );
    let col = get_collection("Shared", "ChatLogs");
    match add_document(col, doc) {
        Ok(_res) => {
            println!("Successfully logged chat item.");
        }
        Err(why) => {
            println!("{:?}", why);
        }
    }

    match name.as_str() {
        "showcase" => {
            //allow specific links

            let items: Vec<Attachment> = msg.attachments.clone();

            if items.len() == 0 {
                match msg.delete(ctx.http) {
                    Ok(()) => {}
                    Err(why) => {
                        println!("Missing permission to delete messages. {:?}", why);
                    }
                }
            };
        }
        "suggestion-box" => {
            if msg.content.starts_with("--sug--") {
                add_reaction(&ctx, &msg, false, 693278870681419867, "upvote");
                add_reaction(&ctx, &msg, false, 693278893121208391, "downvote");
            }
        }
        _ => {}
    }
}

pub fn message_deleted(ctx: &Context, channel_id: ChannelId, msg_id: MessageId) {
    //TODO Link with DB to get old content.
    let logs = get_collection("Shared", "ChatLogs");
    let del_col = get_collection("Shared", "DeletedLogs");

    let message_info = get_document_from_collection(logs, doc! { "messageID": msg_id.to_string() });
    if message_info.is_none() {
        //error here later.
    };
    let message_info = message_info.unwrap();
    let author_field = match message_info.get_str("authorID") {
        Ok(author_id) => author_id,
        Err(_why) => "Failed to gather authorID",
    };
    let content_field = match message_info.get_str("current") {
        Ok(content) => content,
        Err(_why) => "Failed to get content.",
    };

    let name: String = channel_id.name(&ctx).unwrap();

    let guild = get_guild_from_channel_id(&ctx, channel_id);
    if guild.is_none() {
        println!("Guild is none");
        return;
    }
    let guild = guild.unwrap();

    let chat_log = get_channel_from_guild_by_name(&ctx, guild, String::from("chat-log"));
    if chat_log.is_none() {
        println!("Channel is none");
        return;
    }
    let chat_log = chat_log.unwrap();

    let del_doc = create_document_chat_delete(msg_id, author_field, content_field);

    let _ = add_document(del_col, del_doc);

    let _ = chat_log.say(
        &ctx,
        &format!(
            "<@{}> deleted message in {} with content `{}`.",
            author_field, name, content_field
        ),
    );
}

pub fn message_edited(ctx: Context, old_cont: Option<Message>, new_cont: Option<Message>) {}
