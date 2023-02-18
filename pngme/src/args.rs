use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author,version,about,long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands{
    /// Encode message into png specified in file_path
    Encode(EncodeArgs),

    /// Decode message in specified chunk_type from png 
    Decode(DecodeArgs),

    /// Remove chunk from png
    Remove(RemoveArgs),

    /// Print entire png file as chunks
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs{
    pub file_path: String,
    pub chunk_type: String,
    pub message: String,
}

#[derive(Args)]
pub struct DecodeArgs{
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs{
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs{
    pub file_path: String,
}

