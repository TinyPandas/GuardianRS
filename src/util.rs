use serenity::{
    builder::CreateEmbed,
    client::bridge::gateway::ShardManager,
    model::{
        id::{GuildId, ChannelId, MessageId, UserId},
        channel::{Message},
        guild::Member,
        user::OnlineStatus,
    },
    prelude::*,
};

use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct Util;

pub fn get_guild_from_message(msg: &Message) -> Option<GuildId> {
    match msg.guild_id {
        Some(guid) => {
            Some(guid)
        }, None => {
            None
        }
    }
}

pub fn get_message_from_channel_by_id(ctx: &Context, cid: ChannelId, mid: MessageId) -> Option<Message> {
    match cid.message(&ctx, mid) {
        Ok(message) => {
            Some(message)
        }, Err(_why) => {
            None
        }
    }
}

pub fn get_channel_from_guild_by_name(ctx: &Context, guid: GuildId, channel_name: String) -> Option<ChannelId> {
    match guid.channels(&ctx) {
        Ok(channels) => {
            channels.iter().find(|(_, chan)| chan.name == channel_name).map(|(id, _)| *id)
        }, Err(why) => {
            println!("Failed to get channels from {}. {:?}", guid, why);
            None
        }
    }
}

pub fn member_has_role(ctx: &Context, member: &Member, role_name: &String) -> bool {
    match member.roles(&ctx) {
        Some(roles) => {
            for role in roles {
                if role.name.eq(role_name) {
                    return true;
                }
            }
            return false;
        },
        None => {
            return false;
        }
    }
}

pub fn get_members_by_role(ctx: &Context, guid: GuildId, role_name: String) -> Option<Vec<Member>> {
    let mut members = Vec::new();

    for member_result in guid.members_iter(ctx) {
        match member_result {
            Ok(member) => {
                if member_has_role(&ctx, &member, &role_name) {
                    let status = get_member_online_status(&ctx, guid, member.user_id());
                    if !status.is_none() {
                        let status = status.unwrap();
                        match status.name() {
                            "online" => {
                                members.insert(members.len(), member.clone());
                            },
                            _ => {

                            }
                        }
                    }                    
                }
            }, Err(_why) => {
                println!("Errer getting members for role {}.", role_name);
                return None;
            }
        }
    }

    return Some(members);
}

pub fn get_member_online_status(ctx: &Context, guid: GuildId, user_id: UserId) -> Option<OnlineStatus> {
    match guid.to_guild_cached(&ctx) {
        Some(list) => {
            let p = &(*list.read()).presences;

            if p.contains_key(&user_id) {
                match p.get(&user_id) {
                    Some(presence) => {
                        return Some(presence.status);
                    }, None => {
                        return None;
                    }
                }
            }
        }, None => {
            return None;
        }
    };

    return None;
}

pub fn get_guild_from_channel_id(ctx: &Context, cid: ChannelId) -> Option<GuildId> {
    println!("Attempting to get guild from channel {}", cid.to_string());

    let channel = match cid.to_channel(ctx) {
        Ok(channel) => {
            println!("A");
            channel
        }, Err(why) => {
            println!("{:?}", why);
            return None
        }
    };

    let _ = match channel.guild() {
        Some(guild_lock) => {
            println!("B");
            return Some(guild_lock.read().guild_id)
        }, None => {
            println!("Failed part2");
            return None
        }
    };
}

pub fn create_msg_embed(u_id: &str, e: &mut CreateEmbed) {
    e.author(|a| {
        a.name(u_id);

        a
    });
    e.title("Test title");
    e.description("Test Description");
    e.fields(vec![
        ("Field 1", "Value 1", true),
        ("Field 2", "Value 2", true),
        ("Field 3", "Value 3", false)
    ]);
}