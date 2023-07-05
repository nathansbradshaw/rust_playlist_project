mod common;
#[tokio::test]
async fn health_check_works() {
    let (client, address, _) = common::test_util::setup().await;

    let response = client
        .get(&format!("{}/api/health", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
