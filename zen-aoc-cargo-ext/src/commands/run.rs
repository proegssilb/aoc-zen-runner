use crate::{traits::Command, cli::Cli};

pub struct RunCommand{}

impl Command for RunCommand {
    fn new() -> Self {
        RunCommand {  }
    }

    fn run(&mut self, _cli: Cli) -> anyhow::Result<()> {
        todo!("Run not implemented yet.")
    }
}