use super::{Provider, UsageReport};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

pub struct GeminiProvider {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            base_url: "https://generativelanguage.googleapis.com".to_string(),
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

// Google Gemini API doesn't have a standardized, publicly documented /usage endpoint
// similar to OpenAI for general consumption.
// They offer Google AI Studio which is free for reasonable use, and GCP Vertex AI.
// For the sake of this tool, we will make a network request to the closest matching endpoint
// and handle the failure gracefully or return the error.

#[derive(Deserialize, Debug)]
struct GeminiErrorResponse {
    error: GeminiErrorDetail,
}

#[derive(Deserialize, Debug)]
struct GeminiErrorDetail {
    message: String,
}

#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> &'static str {
        "Gemini"
    }

    async fn fetch_today_usage(&self) -> Result<UsageReport> {
        let url = format!("{}/v1beta/models?key={}", self.base_url, self.api_key);

        let response = self.client.get(&url).send().await;

        match response {
            Ok(res) if res.status().is_success() => {
                // If it succeeds, verify the API key is working, but Gemini lacks usage API.
                Ok(UsageReport {
                    provider_name: self.name().to_string(),
                    total_cost: 0.0,
                    model_costs: std::collections::HashMap::new(),
                    error: Some("Gemini API key is valid, but the Usage/Cost API is not implemented for this provider yet".to_string()),
                })
            }
            Ok(res) => {
                let status = res.status();
                let error_msg = if let Ok(err_data) = res.json::<GeminiErrorResponse>().await {
                    err_data.error.message
                } else {
                    format!("HTTP Error: {}", status)
                };

                Ok(UsageReport {
                    provider_name: self.name().to_string(),
                    total_cost: 0.0,
                    model_costs: std::collections::HashMap::new(),
                    error: Some(format!("Gemini API Error: {}", error_msg)),
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
