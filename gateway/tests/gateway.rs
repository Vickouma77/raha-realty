use reqwest::Client;

const BASE: &str = "http://localhost:8000";
const TOKEN: &str = "my-secret-token";

#[tokio::test]
async fn test_health_check() {
    let res = Client::new()
        .get(&format!("{BASE}/health"))
        .send()
        .await;
    assert_eq!(res.unwrap().status(), 200);
}

#[tokio::test]
async fn test_login() {
    let res = Client::new()
        .get(&format!("{BASE}/login"))
        .send()
        .await
        .expect("Failed to login");
    assert_eq!(res.status(), 200);

    let body: serde_json::Value = res.json().await.expect("Invalid Json");
    assert_eq!(body["token"], TOKEN);
}

#[tokio::test]
async fn test_unauthorized_request() {
    let res = Client::new()
        .get(&format!("{BASE}/unauthorized"))
        .send()
        .await
        .expect("Failed to unauthenticated");
    assert_eq!(res.status(), 401);
}

#[tokio::test]
async fn test_authorized_request() {
    let res = Client::new().get(&format!("{BASE}/secure"))
        .header("Authorization", format!("Bearer {}", TOKEN))
        .send().await.expect("Failed");
    assert_eq!(res.status(), 200);
    let body = res.text().await.unwrap();
    assert!(body.contains("Secure data"));
}