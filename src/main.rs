mod fixer;

use serenity::async_trait;
use serenity::builder::EditMessage;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use std::time::Duration;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let fixed_urls = fixer::fix_msg(&msg.content);
        if fixed_urls.is_empty() {
            return;
        }

        let suppress_edit = EditMessage::new().suppress_embeds(true);

        let suppress = msg
            .channel_id
            .edit_message(&ctx.http, msg.id, suppress_edit.clone());
        let reply = async {
            for url in &fixed_urls {
                if let Err(e) = msg.reply(&ctx.http, url).await {
                    eprintln!("Error replying: {e}");
                }
            }
        };

        let (suppress_result, _) = tokio::join!(suppress, reply);
        if let Err(e) = suppress_result {
            eprintln!("Error suppressing embeds: {e}");
        }

        // Ensure embed suppress
        tokio::time::sleep(Duration::from_secs(5)).await;
        if let Err(e) = msg
            .channel_id
            .edit_message(&ctx.http, msg.id, suppress_edit)
            .await
        {
            eprintln!("Error re-suppressing embeds: {e}");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Ready! Logged in as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        eprintln!("Client error: {e}");
    }
}
