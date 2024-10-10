use serenity::all::{
    Context, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, Ready,
};
use serenity::async_trait;
use serenity::model::application::Command;
use serenity::prelude::*;
use std::env;

use crate::commands::activity::Activity;
use crate::commands::command::CommandTrait;
use crate::commands::ping::Ping;
use crate::commands::presence::Presence;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        println!("Interaction: {:?}", interaction);
        if let Interaction::Command(command) = interaction {
            println!("Command: {:?}", command.data.name);
            let content = match command.data.name.as_str() {
                "ping" => Some(Ping.run(&command.data.options, &ctx)),
                "presence" => Some(Presence.run(&command.data.options, &ctx)),
                "activity" => Some(Activity.run(&command.data.options, &ctx)),
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
        /*
        Note: Global commands may take up to an hour to be updated in the user slash commands list.
        If an outdated command data is sent by a user, discord will consider it as an error and then will instantly update that command.
         */
        let commands: Vec<&dyn CommandTrait> = vec![&Ping, &Presence, &Activity];
        for command in commands {
            println!("Registering command: {}", command.name());
            Command::create_global_command(&ctx.http, command.register())
                .await
                .unwrap_or_else(|_| panic!("Failed to register command: {}", command.name()));
        }
    }
}
