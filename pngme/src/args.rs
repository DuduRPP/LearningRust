use clap::Parser;

#[derive(Parser)]
#[command(author,version,about,long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub one: String,
}
