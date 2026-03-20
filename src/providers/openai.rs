use super::{Provider, UsageReport};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{Datelike, Utc};
use serde::Deserialize;
use std::collections::HashMap;

pub struct OpenAIProvider {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            base_url: "https://api.openai.com".to_string(),
        }
    }

    #[cfg(test)]
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            base_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_openai_success() {
        let mut server = Server::new_async().await;

        let mock = server.mock("GET", mockito::Matcher::Regex(r"^/v1/dashboard/billing/usage\?start_date=.*&end_date=.*$".to_string()))
            .with_status(200)
            .with_body(r#"{"total_usage": 150.0}"#)
            .create_async().await;

        let provider = OpenAIProvider::with_base_url("test-key".to_string(), server.url());
        let report = provider.fetch_today_usage().await.unwrap();

        assert_eq!(report.provider_name, "OpenAI");
        assert!(report.error.is_none());
        assert_eq!(report.total_cost, 1.50); // 150 cents = $1.50

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_openai_error() {
        let mut server = Server::new_async().await;

        let mock = server.mock("GET", mockito::Matcher::Regex(r"^/v1/dashboard/billing/usage\?start_date=.*&end_date=.*$".to_string()))
            .with_status(401)
            .with_body(r#"{"error": {"message": "Invalid authentication"}}"#)
            .create_async().await;

        let provider = OpenAIProvider::with_base_url("bad-key".to_string(), server.url());
        let report = provider.fetch_today_usage().await.unwrap();

        assert_eq!(report.provider_name, "OpenAI");
        assert_eq!(report.error, Some("OpenAI Usage API Error: Invalid authentication".to_string()));

        mock.assert_async().await;
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
            "{}/v1/dashboard/billing/usage?start_date={}&end_date={}",
            self.base_url, date_str, date_str
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
