use clap::{Parser, ValueEnum, Subcommand, Args};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<AocSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum AocSubcommand {
    Run(RunArgs),
    Bench(BenchArgs),
    Flamegraph(FlamegraphArgs),
    Input(InputArgs),
    Scaffold(ScaffoldArgs),
    Prep,
    Credentials(CredentialsArgs),
}

#[derive(Debug, Args)]
pub struct RunArgs {
    /// Which year of problem to run. Defaults to the latest published year.
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(2015..))]
    pub year: Option<u32>,

    /// Which day's problem to run. Must be 1-25. Defaults to the current day, clamped to range.
    #[arg(short, long, group = "day-select", value_parser = clap::value_parser!(u8).range(1..=25))]
    pub day: Option<u8>,

    #[arg(long, group = "day-select")]
    pub all_days: bool,

    // Which part number to run. Defaults to both. If specified, must be 1 or 2.
    #[arg(short, long)]
    pub part_num: Option<u8>,
}

#[derive(Debug, Args)]
pub struct BenchArgs {
    /// Which year of problem to run. Defaults to the latest published year.
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(2015..))]
    pub year: Option<u32>,

    /// Which day's problem to run. Must be 1-25. Defaults to the current day, clamped to range.
    #[arg(short, long, group = "day-select", value_parser = clap::value_parser!(u8).range(1..=25))]
    pub day: Option<u8>,

    #[arg(long, group = "day-select")]
    pub all_days: bool,

    // Which part number to run. Defaults to both. If specified, must be 1 or 2.
    #[arg(short, long)]
    pub part_num: Option<u8>,
}

#[derive(Debug, Args)]
pub struct FlamegraphArgs {
    /// Which year of problem to run. Defaults to the latest published year.
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(2015..))]
    pub year: Option<u32>,

    /// Which day's problem to run. Must be 1-25. Defaults to the current day, clamped to range.
    #[arg(short, long, group = "day-select", value_parser = clap::value_parser!(u8).range(1..=25))]
    pub day: Option<u8>,

    #[arg(long, group = "day-select")]
    pub all_days: bool,

    // Which part number to run. Defaults to both. If specified, must be 1 or 2.
    #[arg(short, long)]
    pub part_num: Option<u8>,
}

#[derive(Debug, Args)]
pub struct InputArgs {
    /// Which year of problem to run. Defaults to the latest published year.
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(2015..))]
    pub year: Option<u32>,

    /// Which day's problem to run. Must be 1-25. Defaults to the current day, clamped to range.
    #[arg(short, long, group = "day-select", value_parser = clap::value_parser!(u8).range(1..=25))]
    pub day: Option<u8>,

    #[arg(long, group = "day-select")]
    pub all_days: bool,

    // Which part number to run. Defaults to both. If specified, must be 1 or 2.
    #[arg(short, long)]
    pub part_num: Option<u8>,
}

#[derive(Debug, Args)]
pub struct ScaffoldArgs {
    /// Which year of problem to run. Defaults to the latest published year.
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(2015..))]
    pub year: Option<u32>,

    /// Which day's problem to run. Must be 1-25. Defaults to the current day, clamped to range.
    #[arg(short, long, group = "day-select", value_parser = clap::value_parser!(u8).range(1..=25))]
    pub day: Option<u8>,

    #[arg(long, group = "day-select")]
    pub all_days: bool,

    // Which part number to run. Defaults to both. If specified, must be 1 or 2.
    #[arg(short, long)]
    pub part_num: Option<u8>,
}

#[derive(Debug, Args)]
pub struct CredentialsArgs {
    #[arg(short, long, value_enum)]
    pub source: CredentialSource,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CredentialSource {
    /// Import cookie data frome Chrome/Chromium
    Chrome,
    /// Import cookie data from Firefox
    Firefox,
    /// Manually enter the cookie data
    Manual
}