use serenity::{
    model::{
        id::{GuildId, ChannelId},
        channel::Message,
        guild::Member,
    },
    prelude::*,
};

pub struct Util;

pub fn get_guild_from_message(ctx: &mut Context, msg: &Message) -> Option<GuildId> {
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

pub fn member_has_role(ctx: &mut Context, member: Member, role_name: String) -> bool {
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

    return false
}

pub fn get_staff_members(ctx: &mut Context, guid: &GuildId) -> Result<Vec<Member>> {
    let online_staff = Vec::new();

    for member_result in guid.members_iter(&ctx) {
        match member_result {
            Ok(member) => {
                if (member_has_role(ctx, member, "staff".to_string())) {
                    
                }
            }, Err(why) => {

            }
        }
    }

    return online_staff;
}

pub fn get_user_from_member(ctx: &mut Context, member: Member) -> Result<User> {
    let uid: UserId = member.user_id();
    return uid.to_user(&ctx);
}