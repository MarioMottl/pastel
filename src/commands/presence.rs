use serenity::all::{
    CommandDataOption, CommandOptionType, Context, CreateCommand, CreateCommandOption,
};

use crate::commands::command::CommandTrait;

pub struct Presence;

impl CommandTrait for Presence {
    fn name(&self) -> &'static str {
        "presence"
    }

    fn description(&self) -> &'static str {
        "Sets the presence of the bot"
    }

    fn run(&self, arguments: &[CommandDataOption], ctx: &Context) -> String {
        /*
         Possible statuses:
         - Online
         - Idle
         - Do Not Disturb
         - Invisible
         - Reset Presence ( default: online )
        */

        //TODO: Wait for the commands to update and check the _arguments
        println!("{:?} {:?}", arguments, arguments.len());
        "Placeholder".to_string()
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(Self::name(self))
            .description(Self::description(self))
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "status",
                    "The presence status to set",
                )
                .required(true)
                .add_string_choice("Online", "online")
                .add_string_choice("Idle", "idle")
                .add_string_choice("Do Not Disturb", "dnd")
                .add_string_choice("Invisible", "invisible"),
            )
    }
}
