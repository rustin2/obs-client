use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "obs-client")]
#[command(about = "Headless OBS client using libobs-wrapper", long_about = None)]
pub struct CliArgs {
    /// Path to the recording config file (YAML)
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

pub fn parse() -> CliArgs {
    CliArgs::parse()
}
