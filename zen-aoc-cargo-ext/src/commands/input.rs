use crate::{traits::Command, cli::Cli};

pub struct InputCommand{}

impl Command for InputCommand {
    fn new() -> Self {
        InputCommand {  }
    }

    fn run(&mut self, _cli: Cli) -> anyhow::Result<()> {
        todo!("Input not implemented yet.")
    }
}