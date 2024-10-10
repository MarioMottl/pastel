use crate::commands::command::CommandTrait;
use serenity::all::{CommandDataOption, Context, CreateCommand};

pub struct Ping;

impl CommandTrait for Ping {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn description(&self) -> &'static str {
        "Responds with 'Pong!'"
    }

    fn run(&self, _arguments: &[CommandDataOption], _ctx: &Context) -> String {
        "Pong!".to_string()
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(Self::name(self)).description(Self::description(self))
    }
}
