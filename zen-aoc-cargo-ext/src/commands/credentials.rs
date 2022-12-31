use crate::{traits::Command, cli::Cli};

pub struct CredentialsCommand{}

impl Command for CredentialsCommand {
    fn new() -> Self {
        CredentialsCommand {  }
    }

    fn run(&mut self, _cli: Cli) -> anyhow::Result<()> {
        todo!("Credentials not implemented yet.")
    }
}