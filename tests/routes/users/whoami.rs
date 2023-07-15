#[cfg(test)]
#[sqlx::test]
async fn whoami_return_200(pool: sqlx::Pool<sqlx::Postgres>) {
    let crate::common::types::SetupResponse {
        client, address, ..
    } = crate::common::test_util::setup(pool).await;

    let params = crate::common::types::SignUpUserDto {
        email: "test@examle.com".to_string(),
        password: "12345678".to_string(),
    };

    let signin_response = crate::common::user_test_utils::user_test_utils::create_and_login_user(
        &client, &address, params,
    )
    .await;

    let response = client
        .get(&format!("{}/api/v1/users/whoami", &address))
        .bearer_auth(signin_response.user.access_token.as_ref().unwrap())
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);

    assert_eq!(
        Some(format!(
            "{{\"user\":{{\"email\":\"test@examle.com\",\"access_token\":\"{}\"}}}}",
            signin_response
                .user
                .access_token
                .as_ref()
                .unwrap()
                .to_string()
        )),
        Some(response.text().await.unwrap())
    );
}
