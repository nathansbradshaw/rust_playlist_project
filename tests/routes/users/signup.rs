use crate::common;

#[cfg(test)]
#[sqlx::test]
async fn register_return_400_bad_request(pool: sqlx::Pool<sqlx::Postgres>) {
    let crate::common::types::SetupResponse {
        client, address, ..
    } = crate::common::test_util::setup(pool).await;

    let response = client
        .post(&format!("{}/api/v1/users/signup", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(
        Some("{\"errors\":{\"message\":[\"Missing `Content-Type: application/json` header\"]}}"),
        Some(response.text().await.unwrap().as_str())
    );
}

#[cfg(test)]
#[sqlx::test]
async fn register_return_400_bad_email(pool: sqlx::Pool<sqlx::Postgres>) {
    let crate::common::types::SetupResponse {
        client, address, ..
    } = crate::common::test_util::setup(pool).await;
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
        Some("{\"errors\":{\"message\":[\"Failed to deserialize the JSON body into the target type: email: @examle.com is not a valid user email. at line 1 column 22\"]}}"),
        Some(response.text().await.unwrap().as_str())
    );
}

#[cfg(test)]
#[sqlx::test]
async fn register_return_400_no_email(pool: sqlx::Pool<sqlx::Postgres>) {
    let crate::common::types::SetupResponse {
        client, address, ..
    } = crate::common::test_util::setup(pool).await;

    let params = [("password", "12345678")];
    let response = client
        .post(&format!("{}/api/v1/users/signup", &address))
        .json(&params)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(
        Some("{\"errors\":{\"message\":[\"Failed to deserialize the JSON body into the target type: [0]: invalid type: sequence, expected a string at line 1 column 1\"]}}"),
        Some(response.text().await.unwrap().as_str())
    );
}

#[cfg(test)]
#[sqlx::test]
async fn register_return_400_bad_password(pool: sqlx::Pool<sqlx::Postgres>) {
    use crate::common::types::SignUpUserDto;

    let crate::common::types::SetupResponse {
        client, address, ..
    } = crate::common::test_util::setup(pool).await;
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

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(
        Some("{\"errors\":{\"message\":[\"Failed to deserialize the JSON body into the target type: password: The provided password is invalid at line 1 column 44\"]}}"),
        Some(response.text().await.unwrap().as_str())
    );
}

#[cfg(test)]
#[sqlx::test]
async fn register_return_400_no_password(pool: sqlx::Pool<sqlx::Postgres>) {
    let crate::common::types::SetupResponse {
        client, address, ..
    } = crate::common::test_util::setup(pool).await;
    let params = [("email", "test")];
    let response = client
        .post(&format!("{}/api/v1/users/signup", &address))
        .json(&params)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(
        Some("{\"errors\":{\"message\":[\"Failed to deserialize the JSON body into the target type: [0]: invalid type: sequence, expected a string at line 1 column 1\"]}}"),
        Some(response.text().await.unwrap().as_str())
    );
}

#[cfg(test)]
#[sqlx::test]
async fn register_return_200(pool: sqlx::Pool<sqlx::Postgres>) {
    let crate::common::types::SetupResponse {
        client, address, ..
    } = crate::common::test_util::setup(pool).await;

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
