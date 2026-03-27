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
// For the sake of this tool, we will make a network request to generateContent
// and extract the token count from the usageMetadata in the response.

#[derive(Deserialize, Debug)]
struct GeminiErrorResponse {
    error: GeminiErrorDetail,
}

#[derive(Deserialize, Debug)]
struct GeminiErrorDetail {
    message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeminiGenerateContentResponse {
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeminiUsageMetadata {
    total_token_count: i64,
}

#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> &'static str {
        "Gemini"
    }

    async fn fetch_today_usage(&self) -> Result<UsageReport> {
        let url = format!(
            "{}/v1beta/models/gemini-2.5-flash:generateContent?key={}",
            self.base_url, self.api_key
        );

        let body = serde_json::json!({
            "contents": [{
                "parts": [{"text": "hi"}]
            }]
        });

        let response = self.client.post(&url).json(&body).send().await;

        match response {
            Ok(res) if res.status().is_success() => {
                let mut model_costs = std::collections::HashMap::new();
                if let Ok(data) = res.json::<GeminiGenerateContentResponse>().await {
                    if let Some(usage) = data.usage_metadata {
                        model_costs.insert(
                            "gemini-2.5-flash".to_string(),
                            usage.total_token_count as f64,
                        );
                    }
                }

                Ok(UsageReport {
                    provider_name: self.name().to_string(),
                    total_cost: 0.0,
                    model_costs,
                    error: None,
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
