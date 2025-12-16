use anyhow::Result;
use clap::Parser;

mod cli;
mod engine;
mod output;

use crate::cli::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    engine::run(args)
}
