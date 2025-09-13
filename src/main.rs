use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
// use serenity::model::guild::User;
use serenity::prelude::*;
use sqlx::{Pool, Postgres};

struct Handler {
    db_pool: Pool<Postgres>
}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event. This is called whenever a new message is received.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be
    // dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{} said {} in {}", msg.author.name, msg.content, msg.channel_id);
        let content = msg.content;

        // Borrow content yo
        // Should add some sanitation here? 

        let res = sqlx::query("INSERT INTO messages (user_id, content, channel) VALUES ($1, $2, $3)")
            .bind(msg.author.name.to_string())
            .bind(&content)
            .bind(msg.channel_id.to_string())
            .execute(&self.db_pool)
            .await;
            match res {
                Ok(_) => println!("Message inserted successfully."),
                Err(e) => println!("DB insert failed: {}", e),
            }
        if content == "!ping" {
            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // This ignores the errors, without it it will fail
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let token = std::env::var("DISCORD_API_TOKEN").expect("DISCORD_API_TOKEN not set");
    let db_pool = Pool::<Postgres>::connect(&db_url).await.unwrap();

    let handler = Handler { db_pool };
    // Configure the client with your Discord bot token in the environment.
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
  
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}