use crate::event_handler::Handler;
use log::{error, info};
use serenity::{all::GatewayIntents, Client};
use std::env;
use tokio::signal;

mod commands;
mod event_handler;
mod formatting;
mod logger;

#[tokio::main]
async fn main() {
    logger::logging::setup_logger().expect("Failed to initialize logger");
    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(why) => {
            error!("Failed to get token: {:?}", why);
            return;
        }
    };
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let Handler = Handler::new();

    let mut client = match Client::builder(&token, intents)
        .event_handler(Handler)
        .await
    {
        Ok(client) => {
            info!("Client created successfully.");
            client
        }
        Err(why) => {
            error!("Error creating client: {:?}", why);
            return;
        }
    };

    let client_task = tokio::spawn(async move {
        if let Err(why) = client.start().await {
            error!("Client error: {:?}", why);
        }
    });

    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    info!("Received Ctrl+C, shutting down.");

    client_task.abort();
}
