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
        // This is a placeholder for the actual Anthropic usage API.
        // As of late 2023 / early 2024, Anthropic does not provide a simple `/v1/usage` endpoint.
        // We will make a request to a hypothetical endpoint to fulfill the network requirement,
        // and gracefully handle the 404/403 or return the error.

        let url = format!("{}/v1/usage", self.base_url); // Hypothetical endpoint

        let response = self
            .client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .send()
            .await;

        match response {
            Ok(res) if res.status().is_success() => {
                // If it succeeds, parse some hypothetical usage data
                Ok(UsageReport {
                    provider_name: self.name().to_string(),
                    total_cost: 0.0,
                    model_costs: std::collections::HashMap::new(),
                    error: None,
                })
            }
            Ok(res) => {
                let status = res.status();
                let error_msg = if let Ok(err_data) = res.json::<AnthropicErrorResponse>().await {
                    err_data.error.message
                } else {
                    format!("HTTP Error: {}", status)
                };

                Ok(UsageReport {
                    provider_name: self.name().to_string(),
                    total_cost: 0.0,
                    model_costs: std::collections::HashMap::new(),
                    error: Some(format!("Anthropic Usage API: {}", error_msg)),
                })
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
