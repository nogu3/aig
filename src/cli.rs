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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn test_parse_json_flag() {
        let args = Cli::try_parse_from(["aig", "--json"]).unwrap();
        assert!(args.json);
        assert!(matches!(args.command, None));
    }

    #[test]
    fn test_parse_list_command() {
        let args = Cli::try_parse_from(["aig", "list"]).unwrap();
        assert!(!args.json);
        assert!(matches!(args.command, Some(Commands::List)));
    }

    #[test]
    fn test_parse_list_and_json() {
        let args = Cli::try_parse_from(["aig", "--json", "list"]).unwrap();
        assert!(args.json);
        assert!(matches!(args.command, Some(Commands::List)));
    }
}
