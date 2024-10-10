use log::{error, info};
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

static COMMANDS: &[&'static dyn CommandTrait] = &[&Ping, &Presence, &Activity];

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        info!("Interaction: {:?}", interaction);
        if let Interaction::Command(command) = interaction {
            info!("Command: {:?}", command.data.name);

            let content = COMMANDS
                .iter()
                .find(|&&cmd| cmd.name() == command.data.name)
                .map(|cmd| cmd.run(&command.data.options, &ctx))
                .unwrap_or_else(|| "Unknown command".to_string());

            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                error!("Error sending response: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let deregister_commands = env::var("DEREGISTER_COMMANDS")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        if deregister_commands {
            let global_commands = Command::get_global_commands(&ctx.http).await.unwrap();
            for command in global_commands {
                info!("Deleting command: {}", command.name);
                Command::delete_global_command(&ctx.http, command.id)
                    .await
                    .unwrap_or_else(|_| panic!("Failed to delete command: {}", command.name));
            }
        }
        /*
        Note: Global commands may take up to an hour to be updated in the user slash commands list.
        If an outdated command data is sent by a user, discord will consider it as an error and then will instantly update that command.
         */
        for command in COMMANDS {
            info!("Registering command: {}", command.name());
            Command::create_global_command(&ctx.http, command.register())
                .await
                .unwrap_or_else(|_| panic!("Failed to register command: {}", command.name()));
        }
    }
}
