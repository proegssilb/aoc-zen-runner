use crate::{traits::Command, cli::Cli};

pub struct BenchCommand{}

impl Command for BenchCommand {
    fn new() -> Self {
        BenchCommand {  }
    }

    fn run(&mut self, _cli: Cli) -> anyhow::Result<()> {
        todo!("Bench not implemented yet.")
    }
}