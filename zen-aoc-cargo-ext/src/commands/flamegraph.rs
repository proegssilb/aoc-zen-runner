use crate::{traits::Command, cli::Cli};

pub struct FlamegraphCommand{}

impl Command for FlamegraphCommand {
    fn new() -> Self {
        FlamegraphCommand {  }
    }

    fn run(&mut self, _cli: Cli) -> anyhow::Result<()> {
        todo!("Flamegraph not implemented yet.")
    }
}