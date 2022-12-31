use clap::Parser;
use commands::credentials::CredentialsCommand;

use crate::{cli::Cli, commands::{run::RunCommand, bench::BenchCommand, flamegraph::FlamegraphCommand, input::InputCommand, prep::PrepCommand, scaffold::ScaffoldCommand}, traits::Command};

mod cli;
mod traits;
mod commands;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    println!("Args specified: {:?}", cli);
    let res = match cli.command {
        None => {
            let mut cmd = RunCommand::new();
            cmd.run(cli)
        },
        Some(cli::AocSubcommand::Bench(_)) => {
            let mut cmd = BenchCommand::new();
            cmd.run(cli)
        },
        Some(cli::AocSubcommand::Credentials(_)) => {
            let mut cmd = CredentialsCommand::new();
            cmd.run(cli)
        },
        Some(cli::AocSubcommand::Flamegraph(_)) => {
            let mut cmd = FlamegraphCommand::new();
            cmd.run(cli)
        },
        Some(cli::AocSubcommand::Input(_)) => {
            let mut cmd = InputCommand::new();
            cmd.run(cli)
        },
        Some(cli::AocSubcommand::Prep) => {
            let mut cmd = PrepCommand::new();
            cmd.run(cli)
        },
        Some(cli::AocSubcommand::Run(_)) => {
            let mut cmd = RunCommand::new();
            cmd.run(cli)
        },
        Some(cli::AocSubcommand::Scaffold(_)) => {
            let mut cmd = ScaffoldCommand::new();
            cmd.run(cli)
        },
    };

    res
}
