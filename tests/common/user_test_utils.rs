pub mod user_test_utils {
    use reqwest::Client;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct UserResponse {
        pub user: crate::common::types::ResponseUserDto,
    }

    pub async fn create_and_login_user(
        client: &Client,
        address: &String,
        user_params: crate::common::types::SignUpUserDto,
    ) -> UserResponse {
        client
            .post(&format!("{}/api/v1/users/signup", &address))
            .json(&user_params)
            .send()
            .await
            .expect("Failed to signup");

        client
            .post(&format!("{}/api/v1/users/signin", &address))
            .json(&user_params)
            .send()
            .await
            .expect("Failed to login")
            .json::<UserResponse>()
            .await
            .expect("Unable to deserialize response")
    }
}
