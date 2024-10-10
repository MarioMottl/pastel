use serenity::all::CommandDataOption;
use serenity::builder::CreateCommand;

// Define a trait for commands
pub trait CommandTrait: Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn run(&self, arguments: &[CommandDataOption]) -> String;
    fn register(&self) -> CreateCommand {
        CreateCommand::new(Self::name(self)).description(Self::description(self))
    }
}
