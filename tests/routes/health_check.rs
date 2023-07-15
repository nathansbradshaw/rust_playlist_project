#[cfg(test)]
#[sqlx::test]
async fn health_check_works(pool: sqlx::Pool<sqlx::Postgres>) {
    let crate::common::types::SetupResponse {
        client, address, ..
    } = crate::common::test_util::setup(pool).await;

    let response = client
        .get(&format!("{}/api/v1/health", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
