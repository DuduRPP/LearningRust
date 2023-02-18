pub use anyhow::Result;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub use args::{Commands,Cli, EncodeArgs};
use commands::{encode,print,decode,remove};


fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command{
        Commands::Encode(encode_args) => {
            encode(encode_args)?;
        }
        Commands::Decode(decode_args) => {
            decode(decode_args)?;
        }
        Commands::Remove(remove_args) => {
            remove(remove_args)?;
        }
        Commands::Print(print_args) =>{
            print(print_args)?;
        }
    }

    Ok(())
}
