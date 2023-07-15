#[cfg(test)]
#[sqlx::test]
async fn test_stub(pool: sqlx::Pool<sqlx::Postgres>) {
    let (client, address, _) = crate::common::test_util::setup(pool).await;

    let params = crate::common::types::SignUpUserDto {
        email: "test@examle.com".to_string(),
        password: "12345678".to_string(),
    };

    client
        .post(&format!("{}/api/v1/users/signup", &address))
        .json(&params)
        .send()
        .await
        .expect("Failed to execute request");

    let response = client
        .post(&format!("{}/api/v1/users/signin", &address))
        .json(&params)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);

    // assert_eq!(
    //     Some("{\"user\":{\"email\":\"test@examle.com\",\"access_token\":\"\"}}"),
    //     Some(response.text().await.unwrap().as_str())
    // );
}
