mod common;

#[cfg(test)]
#[tokio::test]
async fn register_return_415_no_form_data() {
    let (client, address, _) = common::test_util::setup().await;

    let response = client
        .post(&format!("{}/api/register", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_client_error());
    assert_eq!(response.status().as_u16(), 415);
    assert_eq!(
        Some("Form requests must have `Content-Type: application/x-www-form-urlencoded`"),
        Some(response.text().await.unwrap().as_str())
    );
}

#[cfg(test)]
#[tokio::test]
async fn register_return_400_bad_email() {
    let (client, address, _) = common::test_util::setup().await;

    let params = [("email", "test"), ("password", "12345678")];

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

#[cfg(test)]
#[tokio::test]
async fn register_return_400_bad_password() {
    let (client, address, _) = common::test_util::setup().await;

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
// #[tokio::test]
// async fn register_return_200() {
//     let (client, address, _) = common::test_util::setup().await;

//     let params = [("email", "test@example.com"), ("password", "12345678")];

//     let response = client
//         .post(&format!("{}/api/register", &address))
//         .form(&params)
//         .send()
//         .await
//         .expect("Failed to execute request");

//     assert!(response.status().is_success());
//     assert_eq!(response.status().as_u16(), 400);
//     assert_eq!(Some(""), Some(response.text().await.unwrap().as_str()));
// }
