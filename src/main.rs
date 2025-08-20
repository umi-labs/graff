mod chart;
mod cli;
mod data;
mod render;
mod spec;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    cli::run(args)
}
