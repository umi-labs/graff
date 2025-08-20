mod cli;
mod spec;
mod data;
mod chart;
mod render;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    cli::run(args)
}
