use crate::{traits::Command, cli::Cli};

pub struct ScaffoldCommand{}

impl Command for ScaffoldCommand {
    fn new() -> Self {
        ScaffoldCommand {  }
    }

    fn run(&mut self, _cli: Cli) -> anyhow::Result<()> {
        todo!("Scaffold not implemented yet.")
    }
}