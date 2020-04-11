mod commands;
pub mod lib;
pub mod util;

use log::{error, info};
use serenity::{
    framework::standard::{
        help_commands,
        macros::{group, help},
        Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
    },
    model::{
        channel::{ReactionType, Message, Attachment},
        event::ResumedEvent,
        gateway::Ready,
        id::{UserId, EmojiId},
    },
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

use commands::{member_commands::*, staff_commands::*};
use lib::ShardManagerContainer;

struct Handler;

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

#[group]
#[commands(ping, info, invite, nocode, request)]
struct General;

#[group]
#[allowed_roles("staff")]
#[commands(shutdown)]
struct Admin;

// The framework provides two built-in help commands for you to use.
// But you can also make your own customized help command that forwards
// to the behaviour of either of them.
#[help]
// This replaces the information that a user can pass
// a command-name as argument to gain specific information about it.
#[individual_command_tip = "Hello! \n\
If you want more information about a specific command, just pass the command as argument."]
// Some arguments require a `{}` in order to replace it with contextual information.
// In this case our `{}` refers to a command's name.
#[command_not_found_text = "Could not find: `{}`."]
// Define the maximum Levenshtein-distance between a searched command-name
// and commands. If the distance is lower than or equal the set distance,
// it will be displayed as a suggestion.
// Setting the distance to 0 will disable suggestions.
#[max_levenshtein_distance(3)]
// When you use sub-groups, Serenity will use the `indention_prefix` to indicate
// how deeply an item is indented.
// The default value is "-", it will be changed to "+".
#[indention_prefix = "+"]
// On another note, you can set up the help-menu-filter-behaviour.
// Here are all possible settings shown on all possible options.
// First case is if a user lacks permissions for a command, we can hide the command.
#[lacking_permissions = "Hide"]
// If the user is nothing but lacking a certain role, we just display it hence our variant is `Nothing`.
#[lacking_role = "Hide"]
// The last `enum`-variant is `Strike`, which ~~strikes~~ a command.
#[wrong_channel = "Strike"]
// Serenity will automatically analyse and generate a hint/tip explaining the possible
// cases of ~~strikethrough-commands~~, but only if
// `strikethrough_commands_tip(Some(""))` keeps `Some()` wrapping an empty `String`, which is the default value.
// If the `String` is not empty, your given `String` will be used instead.
// If you pass in a `None`, no hint will be displayed at all.
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

fn main() {
    // Attempts to load data from .env file
    kankyo::load().expect("Failed to load .env file");

    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment.");

    let mut client = Client::new(&token, Handler).expect("Error creating client.");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix(";"))
            .help(&MY_HELP)
            .group(&GENERAL_GROUP)
            .group(&ADMIN_GROUP),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}