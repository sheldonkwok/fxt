mod fixer;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

use crate::fixer::fix_msg;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, mut msg: Message) {
        let fixed_urls = fix_msg(&msg.content);
        if fixed_urls.is_empty() {
            return;
        }

        // Suppress embeds
        if let Err(e) = msg.suppress_embeds(&ctx.http).await {
            println!("Error suppressing embeds: {:?}", e);
        }

        // Send fixed URLs
        for fixed in fixed_urls {
            if let Err(e) = msg.reply(&ctx.http, fixed).await {
                println!("Error sending message: {:?}", e);
            }
        }

        // Ensure embed suppress after a delay
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        if let Err(e) = msg.suppress_embeds(&ctx.http).await {
            println!("Error suppressing embeds (retry): {:?}", e);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents =
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILDS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
