use indoc::indoc;
use crate::ShardManagerContainer;
use serenity::client::bridge::gateway::ShardId;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

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

    let _ = msg.reply(&ctx, &format!("Pong! {:?}", runner.latency));

    Ok(())
}

#[command]
fn info(ctx: &mut Context, msg: &Message) -> CommandResult {
    let sMsg = indoc!("Hello there, we see you are attempting to ask for help! \
      However, there seems to not be enough informations. We ask that you provide the code you are working with, \
        any errors in the output window, as well as your end goals and what you have tried to get there.
    ");

    let _ = msg.reply(&ctx, sMsg);

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

#[coomand]
fn request(ctx: &mut Context, msg: &Message) -> CommandResult {
    //get reason for request
    //get channel to post in
    //build message with reason and mention staff
    //post message in channel
    //inform requester that # staff were notified.

    Ok(());
}