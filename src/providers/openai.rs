use super::{Provider, UsageReport};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{Datelike, Utc};
use serde::Deserialize;
use std::collections::HashMap;

pub struct OpenAIProvider {
    api_key: String,
    client: reqwest::Client,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct OpenAIUsageResponse {
    // OpenAI usage API format changes over time.
    // Assuming a v1/dashboard/billing/usage endpoint format for costs.
    total_usage: f64,
}

#[derive(Deserialize, Debug)]
struct OpenAIErrorResponse {
    error: OpenAIErrorDetail,
}

#[derive(Deserialize, Debug)]
struct OpenAIErrorDetail {
    message: String,
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "OpenAI"
    }

    async fn fetch_today_usage(&self) -> Result<UsageReport> {
        let now = Utc::now();
        let date_str = format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day());

        // Note: The OpenAI billing/usage endpoint is undocumented and can change.
        // It requires an API key, but sometimes a session key depending on the account type.
        let url = format!(
            "https://api.openai.com/v1/dashboard/billing/usage?start_date={}&end_date={}",
            date_str, date_str
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match response {
            Ok(res) if res.status().is_success() => {
                let usage_data: Result<OpenAIUsageResponse, _> = res.json().await;
                match usage_data {
                    Ok(data) => {
                        let report = UsageReport {
                            provider_name: self.name().to_string(),
                            total_cost: data.total_usage / 100.0, // Convert cents to dollars if it is in cents
                            model_costs: HashMap::new(),
                            error: None,
                        };
                        Ok(report)
                    }
                    Err(e) => Ok(UsageReport {
                        provider_name: self.name().to_string(),
                        total_cost: 0.0,
                        model_costs: HashMap::new(),
                        error: Some(format!("Failed to parse response: {}", e)),
                    }),
                }
            }
            Ok(res) => {
                let status = res.status();
                let error_msg = if let Ok(err_data) = res.json::<OpenAIErrorResponse>().await {
                    err_data.error.message
                } else {
                    format!("HTTP Error: {}", status)
                };

                Ok(UsageReport {
                    provider_name: self.name().to_string(),
                    total_cost: 0.0,
                    model_costs: HashMap::new(),
                    error: Some(format!("OpenAI Usage API Error: {}", error_msg)),
                })
            }
            Err(e) => Ok(UsageReport {
                provider_name: self.name().to_string(),
                total_cost: 0.0,
                model_costs: HashMap::new(),
                error: Some(format!("Request Error: {}", e)),
            }),
        }
    }
}
