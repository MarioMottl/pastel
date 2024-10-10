use serenity::all::{
    Context, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, Ready,
};
use serenity::async_trait;
use serenity::model::application::Command;
use serenity::prelude::*;
use std::env;

use crate::commands::command::CommandTrait;
use crate::commands::ping::Ping;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /*
        TODO: Lookup how you can both have slash commands and message events in the same bot.

        Commands to implement:
        - !ping --> Pong!
        - !help --> List of commands
        - !coinflip heads: string, tails: string, amount: int --> Result of <amount> coinflips
    */
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        println!("Interaction: {:?}", interaction);
        if let Interaction::Command(command) = interaction {
            println!("Command: {:?}", command.data.name);
            let content = match command.data.name.as_str() {
                "ping" => Some(Ping.run(&command.data.options)),
                _ => Some("Unknown command".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Error sending response: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let deregister_commands = env::var("DEREGISTER_COMMANDS")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        if deregister_commands {
            let global_commands = Command::get_global_commands(&ctx.http).await.unwrap();
            for command in global_commands {
                println!("Deleting command: {}", command.name);
                Command::delete_global_command(&ctx.http, command.id)
                    .await
                    .unwrap_or_else(|_| panic!("Failed to delete command: {}", command.name));
            }
        }

        let commands: Vec<&dyn CommandTrait> = vec![&Ping];
        for command in commands {
            println!("Registering command: {}", command.name());
            Command::create_global_command(&ctx.http, command.register())
                .await
                .unwrap_or_else(|_| panic!("Failed to register command: {}", command.name()));
        }
    }
}
