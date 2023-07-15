#[cfg(test)]
#[sqlx::test]
async fn signin_return_200(pool: sqlx::Pool<sqlx::Postgres>) {
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Users {
        user: crate::common::types::ResponseUserDto,
    }
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
        .expect("Failed to signup");

    let signin_response = client
        .post(&format!("{}/api/v1/users/signin", &address))
        .json(&params)
        .send()
        .await
        .expect("Failed to login")
        .json::<Users>()
        .await
        .expect("Unable to deserialize response");

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
