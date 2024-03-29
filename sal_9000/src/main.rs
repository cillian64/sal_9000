use chrono::prelude::*;
use rand::Rng;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::GatewayIntents, gateway::Ready},
    prelude::*,
};
use std::env;
use std::fs;
use std::str::SplitAsciiWhitespace;

mod affix;

struct Handler;

/// Try to respond to a message in the same channel (or DM).  If we can't send
/// the message for whatever reason then just print an error and fail
/// gracefully.
async fn try_respond(msg: &Message, ctx: &Context, response: &str) {
    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
        println!("Error sending message: {:?}", why);
    }
}

async fn cmd_ping(msg: &Message, ctx: &Context, _args: &SplitAsciiWhitespace<'_>) {
    try_respond(&msg, &ctx, "Pong!").await;
}

async fn cmd_roll(msg: &Message, ctx: &Context, args: &mut SplitAsciiWhitespace<'_>) {
    // Choose the roll limits
    let lower: u32 = 1;
    let upper: u32 = match args.next() {
        Some(x) => {
            let x: u32 = match x.parse() {
                Ok(x) => x,
                Err(_) => {
                    try_respond(&msg, &ctx, "That doesn't seem to be a positive number.").await;
                    return;
                }
            };
            if x < 1 {
                try_respond(&msg, &ctx, "The roll limit must be at least 1.").await;
                return;
            }
            x
        }
        None => 100, // Default upper limit of 100 (inclusive)
    };

    // Try to use the user's guild nick, but failing that use their username.
    let username = &msg.author.name;
    let author_name = match msg.guild_id {
        Some(guild_id) => {
            let author_id = msg.author.id;
            match guild_id.member(&ctx.http, author_id).await {
                Ok(member) => match member.nick {
                    Some(nick) => nick,
                    None => username.to_owned(),
                },
                Err(_) => username.to_owned(),
            }
        }
        None => username.to_owned(),
    };

    // Ensure the RNG handle doesn't cross the await - it's not Send.
    let roll: u32 = {
        let mut rng = rand::thread_rng();
        rng.gen_range(lower..(upper + 1))
    };

    let response = format!("{} rolled {} ({})", author_name, roll, upper);
    try_respond(&msg, &ctx, &response).await;
}

fn affix_name_and_emoji(affix_id: &u8) -> String {
    let affix_name = affix::affix_name(*affix_id);
    let affix_emoji = affix::affix_emoji(*affix_id);
    affix_emoji.to_owned() + " " + affix_name
}

async fn cmd_affix(msg: &Message, ctx: &Context, args: &mut SplitAsciiWhitespace<'_>) {
    // Work out whether to do this week or next week
    let (week_str, datetime) = match args.next() {
        Some("this") => ("This", Utc::now()),
        Some("next") => ("Next", Utc::now() + chrono::Duration::weeks(1)),
        Some(_) => {
            try_respond(&msg, &ctx, "Use \"!affix this\" or \"!affix next\"").await;
            return;
        }
        None => ("This", Utc::now()), // Default: this week
    };

    let affixes = affix::get_affixes(datetime);
    let affix_strs: Vec<String> = affixes.iter().map(affix_name_and_emoji).collect();
    let response = week_str.to_owned() + " week's affixes:\n" + &affix_strs.join("\n");
    try_respond(&msg, &ctx, &response).await;
}

#[async_trait]
impl EventHandler for Handler {
    /// Handle `message` events.  Launched concurrently from a threadpool.
    async fn message(&self, ctx: Context, msg: Message) {
        let channel = match msg.channel_id.to_channel(&ctx).await {
            Ok(channel) => channel,
            Err(why) => {
                println!("Error getting channel: {:?}", why);
                return;
            }
        };

        // Determine whether to respond.  We only respond in DMs or #sal-9000
        let respond = match &channel {
            serenity::model::channel::Channel::Guild(guild_channel) => {
                guild_channel.name == "sal-9000"
            }
            serenity::model::channel::Channel::Private(_) => true,
            serenity::model::channel::Channel::Category(_) => false,
            _ => false,
        };

        if !respond {
            println!("Message to unrecognised channel: {:?}", channel);
            return;
        }

        // Now parse the first "command" part of the string
        let mut split = msg.content.trim().split_ascii_whitespace();
        let command = split.next();
        let command = match command {
            Some(command) => command,
            None => return, // Blank message or only whitespace
        };
        if command.chars().nth(0) != Some('!') {
            // Not a command
            return;
        }
        let command = command.split_at(1).1;

        match command {
            "ping" => cmd_ping(&msg, &ctx, &split).await,
            "roll" => cmd_roll(&msg, &ctx, &mut split).await,
            "affix" => cmd_affix(&msg, &ctx, &mut split).await,
            _ => try_respond(&msg, &ctx, "Unknown command.").await,
        };
    }

    /// Run when everything is ready to go.  Context includes data such as
    /// guilds IDs, etc.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    println!("Trying to load discord token from environment...");
    let token = env::var("DISCORD_TOKEN");
    let token = match token {
        Ok(token) => token,
        Err(_) => {
            println!("Env variable not found, trying to load discord token from file...");
            let token = fs::read_to_string("discord_token.txt");
            match token {
                Ok(token) => token.trim().to_owned(),
                Err(_) => {
                    println!("Could not read discord_token.txt.  Aborting.");
                    return;
                }
            }
        }
    };
    println!("Token loaded successfully.");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
