use crate::{traits::Command, cli::Cli};

pub struct PrepCommand{}

impl Command for PrepCommand {
    fn new() -> Self {
        PrepCommand {  }
    }

    fn run(&mut self, _cli: Cli) -> anyhow::Result<()> {
        todo!("Prep not implemented yet.")
    }
}