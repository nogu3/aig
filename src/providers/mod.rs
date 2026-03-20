use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use std::collections::HashMap;

pub mod anthropic;
pub mod gemini;
pub mod openai;

#[derive(Debug, Serialize, Default)]
pub struct UsageReport {
    pub provider_name: String,
    pub total_cost: f64,
    pub model_costs: HashMap<String, f64>,
    pub error: Option<String>,
}

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &'static str;
    async fn fetch_today_usage(&self) -> Result<UsageReport>;
}
