use crate::database::*;
use crate::util::*;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[allowed_roles("staff")]
fn shutdown(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        manager.lock().shutdown_all();
    } else {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

        return Ok(());
    }

    let _ = msg.reply(&ctx, "Shutting down!");

    Ok(())
}

#[command]
#[allowed_roles("staff")]
fn test_embed(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx, |m| {
        m.content("Test embed");
        m.embed(|e| {
            create_msg_embed(&msg.author.name, e);
            e
        });

        m
    });

    Ok(())
}

fn ban() {}
fn filter() {}
fn history() {}
fn kick() {}
fn mute() {}
fn offtopic() {}
fn removemute() {}
fn rules() {}
fn unmute() {}
fn warn() {}
