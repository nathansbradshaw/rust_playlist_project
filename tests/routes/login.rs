use crate::common;

#[cfg(test)]
#[sqlx::test]
async fn login_user_can_login(pool: sqlx::Pool<sqlx::Postgres>) {
    let (client, address, _) = common::test_util::setup(pool).await;

    let params = [("email", "test@example.com"), ("password", "12345678")];

    // Act
    let _ = client
        .post(&format!("{}/api/register", &address))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request");

    let response = client
        .post(&format!("{}/api/login", &address))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 400);
}
