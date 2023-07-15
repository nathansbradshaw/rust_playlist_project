#[cfg(test)]
#[sqlx::test]
async fn update_refresh_token_200(pool: sqlx::Pool<sqlx::Postgres>) {
    let crate::common::types::SetupResponse {
        client, address, ..
    } = crate::common::test_util::setup(pool).await;

    let params = crate::common::types::SignUpUserDto {
        email: "test@examle.com".to_string(),
        password: "12345678".to_string(),
    };
    let new_password = crate::common::types::SignUpUserDto {
        email: "test@examle.com".to_string(),
        password: "newnewnewnew".to_string(),
    };

    let signin_response = crate::common::user_test_utils::user_test_utils::create_and_login_user(
        &client, &address, params,
    )
    .await;

    let response = client
        .get(&format!("{}/api/v1/users/refresh", &address))
        .bearer_auth(signin_response.user.access_token.as_ref().unwrap())
        .json(&new_password)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);
    //TODO access token should be updated

    // let response_data = &response
    //     .json::<crate::common::user_test_utils::user_test_utils::UserResponse>()
    //     .await
    //     .expect("Unable to deserialize response");
    // assert_ne!(
    //     signin_response.user.access_token,
    //     response_data.user.access_token
    // );
}
