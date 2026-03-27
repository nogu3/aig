use aig::providers::{
    anthropic::AnthropicProvider, gemini::GeminiProvider, openai::OpenAIProvider, Provider,
};
use mockito::Server;

// --- Anthropic Tests ---

#[tokio::test]
async fn test_anthropic_success() {
    let mut server = Server::new_async().await;
    // Auth is valid, so it returns 400 Bad Request because messages is empty
    let mock = server
        .mock("POST", "/v1/messages")
        .with_status(400)
        .create_async()
        .await;

    let provider = AnthropicProvider::with_base_url("test-key".to_string(), server.url());
    let report = provider.fetch_today_usage().await.unwrap();

    assert_eq!(report.provider_name, "Anthropic");
    assert!(report
        .error
        .unwrap()
        .contains("Usage/Cost API is not publicly available"));
    assert_eq!(report.total_cost, 0.0);

    mock.assert_async().await;
}

#[tokio::test]
async fn test_anthropic_error() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/messages")
        .with_status(401)
        .with_body(r#"{"error": {"message": "Invalid API Key"}}"#)
        .create_async()
        .await;

    let provider = AnthropicProvider::with_base_url("bad-key".to_string(), server.url());
    let report = provider.fetch_today_usage().await.unwrap();

    assert_eq!(report.provider_name, "Anthropic");
    assert_eq!(
        report.error,
        Some("Anthropic API Error: Invalid API Key".to_string())
    );

    mock.assert_async().await;
}

// --- Gemini Tests ---

#[tokio::test]
async fn test_gemini_success() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1beta/models?key=test-key")
        .with_status(200)
        .create_async()
        .await;

    let provider = GeminiProvider::with_base_url("test-key".to_string(), server.url());
    let report = provider.fetch_today_usage().await.unwrap();

    assert_eq!(report.provider_name, "Gemini");
    assert!(report.error.unwrap().contains("Gemini API key is valid"));
    assert_eq!(report.total_cost, 0.0);

    mock.assert_async().await;
}

#[tokio::test]
async fn test_gemini_error() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/v1beta/models?key=bad-key")
        .with_status(403)
        .with_body(r#"{"error": {"message": "API key not valid"}}"#)
        .create_async()
        .await;

    let provider = GeminiProvider::with_base_url("bad-key".to_string(), server.url());
    let report = provider.fetch_today_usage().await.unwrap();

    assert_eq!(report.provider_name, "Gemini");
    assert_eq!(
        report.error,
        Some("Gemini API Error: API key not valid".to_string())
    );

    mock.assert_async().await;
}

// --- OpenAI Tests ---

#[tokio::test]
async fn test_openai_success() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"^/v1/dashboard/billing/usage\?start_date=.*&end_date=.*$".to_string(),
            ),
        )
        .with_status(200)
        .with_body(r#"{"total_usage": 150.0}"#)
        .create_async()
        .await;

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

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"^/v1/dashboard/billing/usage\?start_date=.*&end_date=.*$".to_string(),
            ),
        )
        .with_status(401)
        .with_body(r#"{"error": {"message": "Invalid authentication"}}"#)
        .create_async()
        .await;

    let provider = OpenAIProvider::with_base_url("bad-key".to_string(), server.url());
    let report = provider.fetch_today_usage().await.unwrap();

    assert_eq!(report.provider_name, "OpenAI");
    assert_eq!(
        report.error,
        Some("OpenAI Usage API Error: Invalid authentication".to_string())
    );

    mock.assert_async().await;
}
