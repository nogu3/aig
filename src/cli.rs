use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "aig",
    about = "AI Gauge: Tracks and reports API cost/usage for LLM providers"
)]
pub struct Cli {
    /// Output results in JSON format
    #[arg(long)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Shows breakdown per model/provider
    List,
}
