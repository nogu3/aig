use aig::cli::{Cli, Commands};
use clap::Parser;

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
