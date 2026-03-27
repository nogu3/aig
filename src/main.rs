use aig::cli::{Cli, Commands};
use aig::config::Config;
use aig::providers::{
    anthropic::AnthropicProvider, gemini::GeminiProvider, openai::OpenAIProvider, Provider,
    UsageReport,
};
use clap::Parser;
use futures::future::join_all;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let config = Config::load();

    let mut configured_providers: Vec<Arc<dyn Provider>> = Vec::new();

    if let Some(c) = config.anthropic {
        if let Some(key) = c.api_key {
            configured_providers.push(Arc::new(AnthropicProvider::new(key)));
        }
    }

    if let Some(c) = config.gemini {
        if let Some(key) = c.api_key {
            configured_providers.push(Arc::new(GeminiProvider::new(key)));
        }
    }

    if let Some(c) = config.openai {
        if let Some(key) = c.api_key {
            configured_providers.push(Arc::new(OpenAIProvider::new(key)));
        }
    }

    if configured_providers.is_empty() {
        eprintln!("Warning: No providers configured. Please set API keys via environment variables or config file.");
        std::process::exit(1);
    }

    let fetch_futures = configured_providers.iter().map(|provider| {
        let p = Arc::clone(provider);
        async move { p.fetch_today_usage().await }
    });

    let results = join_all(fetch_futures).await;

    let mut reports: Vec<UsageReport> = Vec::new();
    let mut total_cost = 0.0;
    let mut has_errors = false;

    for result in results {
        match result {
            Ok(report) => {
                total_cost += report.total_cost;
                if report.error.is_some() {
                    has_errors = true;
                }
                reports.push(report);
            }
            Err(e) => {
                eprintln!("Error fetching usage: {}", e);
                has_errors = true;
            }
        }
    }

    if args.json {
        let output = serde_json::to_string_pretty(&reports)?;
        println!("{}", output);
    } else {
        match args.command {
            Some(Commands::List) => {
                for report in &reports {
                    println!("{}:", report.provider_name);
                    if let Some(ref err) = report.error {
                        eprintln!("  Error: {}", err);
                    } else {
                        println!("  Total Cost: ${:.4}", report.total_cost);
                        for (model, cost) in &report.model_costs {
                            println!("    {}: ${:.4}", model, cost);
                        }
                    }
                }
                println!("\nTotal Cost (All Providers): ${:.4}", total_cost);
            }
            None => {
                // Simple summary
                println!("Today's Total Cost: ${:.4}", total_cost);
                for report in &reports {
                    if let Some(ref err) = report.error {
                        eprintln!("Error from {}: {}", report.provider_name, err);
                    }
                }
            }
        }
    }

    if has_errors {
        std::process::exit(1);
    }

    Ok(())
}
