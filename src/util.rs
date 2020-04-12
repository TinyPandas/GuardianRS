use serenity::{
    model::{
        id::{GuildId, ChannelId},
        channel::Message,
        guild::Member,
    },
    prelude::*,
};

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

pub fn get_channel_from_guild_by_name(ctx: &mut Context, guid: &GuildId, channel_name: String) -> Option<ChannelId> {
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

pub fn get_members_by_role(ctx: &Context, guid: &GuildId, role_name: String) -> Option<Vec<Member>> {
    let mut members = Vec::new();

    for member_result in guid.members_iter(ctx) {
        match member_result {
            Ok(member) => {
                if member_has_role(&ctx, &member, &role_name) {
                    // check if member is only.
                    members.insert(members.len(), member.clone());
                }
            }, Err(_why) => {
                println!("Errer getting members for role {}.", role_name);
                return None;
            }
        }
    }

    return Some(members);
}