#[cfg(test)]
#[sqlx::test]
async fn signout_return_200(pool: sqlx::Pool<sqlx::Postgres>) {
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
        .post(&format!("{}/api/v1/users/signout", &address))
        .bearer_auth(signin_response.user.access_token.as_ref().unwrap())
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);

    assert_eq!(Some("".to_string()), Some(response.text().await.unwrap()));
}

//TODO delete session on logout
// #[cfg(test)]
// #[sqlx::test]
// async fn signout_invalidates_session_token(pool: sqlx::Pool<sqlx::Postgres>) {
//     let crate::common::types::SetupResponse {
//         client, address, ..
//     } = crate::common::test_util::setup(pool).await;

//     let params = crate::common::types::SignUpUserDto {
//         email: "test@examle.com".to_string(),
//         password: "12345678".to_string(),
//     };

//     let signin_response = crate::common::user_test_utils::user_test_utils::create_and_login_user(
//         &client, &address, params,
//     )
//     .await;

//     client
//         .post(&format!("{}/api/v1/users/signout", &address))
//         .bearer_auth(signin_response.user.access_token.as_ref().unwrap())
//         .send()
//         .await
//         .expect("Failed to execute request");

//     let response = client
//         .get(&format!("{}/api/v1/users/whoami", &address))
//         .bearer_auth(signin_response.user.access_token.as_ref().unwrap())
//         .send()
//         .await
//         .expect("Failed to execute request");

//     // assert!(response.status().is_success());
//     assert_eq!(response.status().as_u16(), 401);

//     assert_eq!(Some("".to_string()), Some(response.text().await.unwrap()));
// }
