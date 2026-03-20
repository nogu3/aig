use aig::config::Config;

#[test]
fn test_deserialize_config() {
    let toml_content = r#"
        [anthropic]
        api_key = "test-anthropic-key"

        [gemini]
        api_key = "test-gemini-key"

        [openai]
        api_key = "test-openai-key"
    "#;

    let config: Config = toml::from_str(toml_content).unwrap();

    assert_eq!(config.anthropic.unwrap().api_key, Some("test-anthropic-key".to_string()));
    assert_eq!(config.gemini.unwrap().api_key, Some("test-gemini-key".to_string()));
    assert_eq!(config.openai.unwrap().api_key, Some("test-openai-key".to_string()));
}

#[test]
fn test_deserialize_partial_config() {
    let toml_content = r#"
        [openai]
        api_key = "test-openai-key"
    "#;

    let config: Config = toml::from_str(toml_content).unwrap();

    assert!(config.anthropic.is_none());
    assert!(config.gemini.is_none());
    assert_eq!(config.openai.unwrap().api_key, Some("test-openai-key".to_string()));
}
