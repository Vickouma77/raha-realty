use gateway::config::AppState;
use tokio;

#[tokio::test]
async fn test_read_service_config() {
    let app_state = AppState::new();
    let services = app_state.services.read().await;

    assert!(services.contains_key("properties"));
    let config = services.get("properties").unwrap();
    assert_eq!(config.instance[0].url, "http://localhost:8001/properties");
}
