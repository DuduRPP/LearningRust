pub use anyhow::Result;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("one: {:?}", cli.one);
    Ok(())
}
