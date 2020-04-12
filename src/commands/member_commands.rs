use indoc::indoc;
use serenity::client::bridge::gateway::ShardId;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::lib::ShardManagerContainer;
use crate::util::*;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

            return Ok(());
        }
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.reply(&ctx, "No shard found");

            return Ok(());
        }
    };

    let reply = match runner.latency {
        None => "Pong! [Still awaiting first heartbeat.]".to_string(),
        Some(latency) => format!("Pong {:?}", latency),
    };

    let _ = msg.reply(&ctx, reply);

    Ok(())
}

#[command]
fn info(ctx: &mut Context, msg: &Message) -> CommandResult {
    let s_msg = indoc!("Hello there, we see you are attempting to ask for help! \
      However, there seems to not be enough informations. We ask that you provide the code you are working with, \
        any errors in the output window, as well as your end goals and what you have tried to get there.
    ");

    let _ = msg.reply(&ctx, s_msg);

    Ok(())
}

#[command]
fn invite(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(&ctx.http, "Invite your friends using: https://discord.gg/WHTAYrK");

    Ok(())
}

#[command]
fn nocode(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(&ctx, "To get assistance, we trecommend you provide the code you are working with inside of a codeblock. \n \\`\\`\\`lua \n --code \n \\`\\`\\`");

    Ok(())
}

#[command]
fn status(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild = get_guild_from_message(&msg);
    if guild.is_none() {
        let _ = msg.reply(&ctx, "Failed to get your status.");
        return Ok(());
    }
    let guild = guild.unwrap();

    let user_id = msg.author.id;
    let indiv_status = get_member_online_status(ctx, guild, user_id);
    if indiv_status.is_none() {
        let _ = msg.reply(&ctx, "Failed to get your status.");
    }
    let indiv_status = indiv_status.unwrap();

    let _ = msg.reply(&ctx, &format!("Your online status is: {}", indiv_status.name()));

    Ok(())
}

#[command]
fn request(ctx: &mut Context, msg: &Message) -> CommandResult {
    //get reason for request
    let reason = &msg.content[9..];
    
    //get guild
    let guild = get_guild_from_message(&msg);
    if guild.is_none() {
        let _ = msg.reply(&ctx, "Failed to mention staff.");
        return Ok(());
    }
    let guild = guild.unwrap();

    //get channel to post in
    let channel = get_channel_from_guild_by_name(ctx, guild, String::from("commands"));
    if channel.is_none() {
        let _ = msg.reply(&ctx, "Failed to mention staff.");
        return Ok(());
    }
    let channel = channel.unwrap();

    let online_staff = get_members_by_role(ctx, guild, String::from("staff"));
    if online_staff.is_none() {
        let _ = msg.reply(&ctx, "Failed to mention staff.");
        return Ok(());
    }
    let online_staff = online_staff.unwrap();
    let mut staff_as_mention = "".to_string();

    for member in &online_staff {
        staff_as_mention.push_str(&member.mention());
        staff_as_mention.push_str(", ");
    }
    
    //build message with reason and mention staff
    //post message in channel
    let _ = channel.say(&ctx, &format!("{} \n {} has request staff in {} for {}.", staff_as_mention, msg.author.mention(), msg.channel_id.mention(), &reason));
    
    //inform requester that # staff were notified.
    let _ = msg.reply(&ctx, &format!("Mentioned {} staff.", online_staff.len()));

    Ok(())
}