use serenity::all::{
    CommandDataOption, CommandOptionType, Context, CreateCommand, CreateCommandOption,
};

use crate::commands::command::CommandTrait;

pub struct Activity;

impl CommandTrait for Activity {
    fn name(&self) -> &'static str {
        "activity"
    }

    fn description(&self) -> &'static str {
        "Sets the activity of the bot"
    }

    fn run(&self, arguments: &[CommandDataOption], ctx: &Context) -> String {
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
                    "activity",
                    "The activity status to set",
                )
                .required(true)
                .add_string_choice("Game", "CHANGE_ME"),
            )
    }
}
