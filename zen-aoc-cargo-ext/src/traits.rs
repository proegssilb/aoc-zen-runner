use anyhow::Result;

use crate::cli::Cli;

pub trait Command {
    fn new() -> Self;
    fn run(&mut self, cli: Cli) -> Result<()>;
}