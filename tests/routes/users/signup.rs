use crate::common;

#[cfg(test)]
#[sqlx::test]
async fn register_return_400_bad_request(pool: sqlx::Pool<sqlx::Postgres>) {
    let (client, address, _) = common::test_util::setup(pool).await;

    let response = client
        .post(&format!("{}/api/v1/users/signup", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(Some("{\"errors\":{\"message\":[\"Expected request with `Content-Type: application/json`\"]}}"), Some(response.text().await.unwrap().as_str()));
}

#[cfg(test)]
#[sqlx::test]
async fn register_return_400_bad_email(pool: sqlx::Pool<sqlx::Postgres>) {
    let (client, address, _) = common::test_util::setup(pool).await;
    let params = common::types::SignUpUserDto {
        email: "@examle.com".to_string(),
        password: "123456789".to_string(),
    };

    let response = client
        .post(&format!("{}/api/v1/users/signup", &address))
        .json(&params)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(
        Some("{\"errors\":{\"message\":[\" email: @examle.com is not a valid user email.\"]}}"),
        Some(response.text().await.unwrap().as_str())
    );
}

#[cfg(test)]
#[sqlx::test]
async fn register_return_400_bad_password(pool: sqlx::Pool<sqlx::Postgres>) {
    use crate::common::types::SignUpUserDto;

    let (client, address, _) = common::test_util::setup(pool).await;

    // let params = [("email", "test@example.com"), ("password", "123")];
    let params = SignUpUserDto {
        email: "test@examle.com".to_string(),
        password: "123".to_string(),
    };

    let response = client
        .post(&format!("{}/api/v1/users/signup", &address))
        .json(&params)
        .send()
        .await
        .expect("Failed to execute request");

    // assert!(response.status().is_client_error());
    // assert_eq!(response.status().as_u16(), 400);
    assert_eq!(
        Some("{\"errors\":{\"message\":[\"Invalid password\"]}}"),
        Some(response.text().await.unwrap().as_str())
    );
}

#[cfg(test)]
#[sqlx::test]
async fn register_return_200(pool: sqlx::Pool<sqlx::Postgres>) {
    let (client, address, _) = common::test_util::setup(pool).await;

    let params = common::types::SignUpUserDto {
        email: "test@examle.com".to_string(),
        password: "12345678".to_string(),
    };

    let response = client
        .post(&format!("{}/api/v1/users/signup", &address))
        .json(&params)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(
        Some("{\"user\":{\"email\":\"test@examle.com\",\"access_token\":\"\"}}"),
        Some(response.text().await.unwrap().as_str())
    );
}
