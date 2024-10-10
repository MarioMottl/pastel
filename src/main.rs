use crate::event_handler::Handler;
use serenity::{all::GatewayIntents, Client};
use std::env;
use tokio::signal;

mod commands;
mod event_handler;
mod formatting;

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Spawn the client in a separate task
    let client_task = tokio::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    });

    // Listen for the Ctrl+C signal
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    println!("Received Ctrl+C, shutting down.");

    // Gracefully shut down the client
    client_task.abort();
}
