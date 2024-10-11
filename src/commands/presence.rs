use crate::commands::command::CommandTrait;
use serenity::all::{
    CommandDataOption, CommandDataOptionValue, CommandOptionType, Context, CreateCommand,
    CreateCommandOption,
};

pub struct Presence;

impl CommandTrait for Presence {
    fn name(&self) -> &'static str {
        "presence"
    }

    fn description(&self) -> &'static str {
        "Sets the presence of the bot"
    }

    // See: https://docs.rs/serenity/latest/serenity/client/struct.Context.html#method.online
    fn run(&self, arguments: &[CommandDataOption], ctx: &Context) -> String {
        /*
         Possible statuses:
         - Online
         - Idle
         - Do Not Disturb
         - Invisible
         - Reset Presence ( default: online )

        [CommandDataOption { name: "presence", value: String("dnd") }] 1
        [CommandDataOption { name: "presence", value: String("online") }] 1
        [CommandDataOption { name: "presence", value: String("idle") }] 1
        [CommandDataOption { name: "presence", value: String("invisible") }] 1
        */

        if let Some(option) = arguments.iter().find(|opt| opt.name == "presence") {
            if let CommandDataOptionValue::String(presence) = &option.value {
                println!("Setting presence to: {}", presence);
                match presence.as_str() {
                    "online" => ctx.online(),
                    "idle" => ctx.idle(),
                    "dnd" => ctx.dnd(),
                    "invisible" => ctx.invisible(),
                    _ => ctx.online(),
                }
            }
        }
        "Setting presence".to_string()
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(Self::name(self))
            .description(Self::description(self))
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "presence",
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
