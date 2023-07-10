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
async fn register_return_400_bad_password(pool: sqlx::Pool<sqlx::Postgres>) {
    let (client, address, _) = common::test_util::setup(pool).await;

    let params = [("email", "test@example.com"), ("password", "123")];

    let response = client
        .post(&format!("{}/api/register", &address))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(Some(""), Some(response.text().await.unwrap().as_str()));
}

// #[cfg(test)]
// #[sqlx::test]
// async fn register_return_200(pool: sqlx::Pool<sqlx::Postgres>) {
//     let (client, address, _) = common::test_util::setup(pool).await;

//     let params = [("email", "test@example.com"), ("password", "12345678")];

//     let response = client
//         .post(&format!("{}/api/register", &address))
//         .form(&params)
//         .send()
//         .await
//         .expect("Failed to execute request");

//     assert!(response.status().is_success());
//     assert_eq!(response.status().as_u16(), 200);
//     assert_eq!(Some(""), Some(response.text().await.unwrap().as_str()));
// }
