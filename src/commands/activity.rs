use crate::commands::command::CommandTrait;
use serenity::all::{
    CommandDataOption, CommandDataOptionValue, CommandOptionType, Context, CreateCommand,
    CreateCommandOption,
};
use serenity::gateway::ActivityData;

pub struct Activity;

impl CommandTrait for Activity {
    fn name(&self) -> &'static str {
        "activity"
    }

    fn description(&self) -> &'static str {
        "Sets the activity of the bot"
    }

    // See: https://docs.rs/serenity/latest/serenity/client/struct.Context.html#method.online
    fn run(&self, arguments: &[CommandDataOption], ctx: &Context) -> String {
        /*
        [CommandDataOption { name: "activity", value: String("Stafield") }]
        */

        if let Some(option) = arguments.iter().find(|opt| opt.name == "activity") {
            if let CommandDataOptionValue::String(activity) = &option.value {
                println!("Setting activity to: {}", activity);
                ctx.set_activity(Some(ActivityData::playing(activity)));
            }
        }

        "Setting activity".to_string()
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
                .required(true),
            )
    }
}
