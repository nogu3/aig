use super::{Provider, UsageReport};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

pub struct AnthropicProvider {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            base_url: "https://api.anthropic.com".to_string(),
        }
    }

    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            base_url,
        }
    }
}

// Anthropic doesn't have a standardized, publicly documented /usage endpoint
// similar to OpenAI for general consumption without an organization ID.
// However, there is a known workspace usage API endpoint that some integrations use.
// Format: https://api.anthropic.com/v1/organizations/{org_id}/workspaces/{workspace_id}/usage
// Given the lack of organization ID in the config, we'll implement a best-effort
// attempt to fetch tokens or return a descriptive error that fits the tool's requirements.
// For now, we will attempt to hit an assumed `/v1/usage` endpoint, and handle the failure gracefully.

#[derive(Deserialize, Debug)]
struct AnthropicErrorResponse {
    error: AnthropicErrorDetail,
}

#[derive(Deserialize, Debug)]
struct AnthropicErrorDetail {
    message: String,
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &'static str {
        "Anthropic"
    }

    async fn fetch_today_usage(&self) -> Result<UsageReport> {
        // As of 2024, Anthropic does not provide a simple `/v1/usage` endpoint for standard API keys.
        // To prevent a generic "404 Not Found" error, we'll make a deliberate minimal request
        // to `/v1/messages` just to validate the API key (which returns 401 if invalid, 400 if valid but empty).
        let url = format!("{}/v1/messages", self.base_url);

        // A deliberate malformed request to verify authentication
        let body = serde_json::json!({
            "model": "claude-3-haiku-20240307",
            "max_tokens": 1,
            "messages": []
        });

        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await;

        match response {
            Ok(res) => {
                let status = res.status();

                // If auth is valid, Anthropic will likely return a 400 Bad Request because messages is empty
                if status.is_client_error() && status.as_u16() == 401 {
                    let error_msg = if let Ok(err_data) = res.json::<AnthropicErrorResponse>().await
                    {
                        err_data.error.message
                    } else {
                        "Invalid API Key".to_string()
                    };

                    Ok(UsageReport {
                        provider_name: self.name().to_string(),
                        total_cost: 0.0,
                        model_costs: std::collections::HashMap::new(),
                        error: Some(format!("Anthropic API Error: {}", error_msg)),
                    })
                } else {
                    // Auth succeeded (or endpoint exists), but Usage API is not implemented
                    Ok(UsageReport {
                        provider_name: self.name().to_string(),
                        total_cost: 0.0,
                        model_costs: std::collections::HashMap::new(),
                        error: Some("Anthropic API key is valid, but the Usage/Cost API is not publicly available for this provider yet".to_string()),
                    })
                }
            }
            Err(e) => Ok(UsageReport {
                provider_name: self.name().to_string(),
                total_cost: 0.0,
                model_costs: std::collections::HashMap::new(),
                error: Some(format!("Request Error: {}", e)),
            }),
        }
    }
}
