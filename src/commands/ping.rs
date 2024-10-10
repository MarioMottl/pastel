use serenity::all::CommandDataOption;

use crate::commands::command::CommandTrait;

pub struct Ping;

impl CommandTrait for Ping {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn description(&self) -> &'static str {
        "Responds with 'Pong!'"
    }

    fn run(&self, _arguments: &[CommandDataOption]) -> String {
        "Pong!".to_string()
    }

    fn register(&self) -> serenity::all::CreateCommand {
        serenity::all::CreateCommand::new(Self::name(self)).description(Self::description(self))
    }
}
