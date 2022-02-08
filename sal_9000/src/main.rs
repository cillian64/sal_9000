use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

/// Try to respond to a message in the same channel (or DM).  If we can't send
/// the message for whatever reason then just print an error and fail
/// gracefully.
async fn try_respond(msg: &Message, ctx: &Context, response: &str) {
    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
        println!("Error sending message: {:?}", why);
    }
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

        // Determine whether to respond.  We only respond in DMs or #hal
        let respond = match &channel {
            serenity::model::channel::Channel::Guild(guild_channel) => guild_channel.name == "hal",
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
            "ping" => try_respond(&msg, &ctx, "Pong!").await,
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
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
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
