use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub anthropic: Option<AnthropicConfig>,
    pub gemini: Option<GeminiConfig>,
    pub openai: Option<OpenAIConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub struct AnthropicConfig {
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct GeminiConfig {
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct OpenAIConfig {
    pub api_key: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let mut config = Config::default();

        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("aig").join("config.toml");
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(config_path) {
                    if let Ok(file_config) = toml::from_str::<Config>(&content) {
                        config = file_config;
                    }
                }
            }
        }

        // Environment variables override config file
        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            let mut anthropic = config.anthropic.unwrap_or_default();
            anthropic.api_key = Some(key);
            config.anthropic = Some(anthropic);
        }

        if let Ok(key) = std::env::var("GEMINI_API_KEY") {
            let mut gemini = config.gemini.unwrap_or_default();
            gemini.api_key = Some(key);
            config.gemini = Some(gemini);
        }

        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            let mut openai = config.openai.unwrap_or_default();
            openai.api_key = Some(key);
            config.openai = Some(openai);
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
