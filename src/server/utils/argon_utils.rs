use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
// use secrecy::{ExposeSecret, Secret};

use std::sync::Arc;

// use crate::config::AppConfig;
use crate::server::error::{AppResult, Error};

/// A security service for handling JWT authentication.
pub type DynArgonUtil = Arc<dyn ArgonUtil + Send + Sync>;

pub trait ArgonUtil {
    fn hash_password(&self, raw_password: &str) -> AppResult<String>;

    fn verify_password(
        &self,
        stored_password: Option<String>,
        attempted_pasword: String,
    ) -> AppResult<bool>;
}

pub struct ArgonSecurityUtil {
    // config: Arc<AppConfig>,
}

impl ArgonSecurityUtil {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for ArgonSecurityUtil {
    fn default() -> Self {
        Self::new()
    }
}

impl ArgonUtil for ArgonSecurityUtil {
    fn hash_password(&self, raw_password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(raw_password.as_bytes(), &salt)
        .context("Error hashing password")
        .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?
        .to_string();

        // change this to secret
        Ok(password_hash)
    }

    fn verify_password(
        &self,
        stored_password: Option<String>,
        attempted_pasword: String,
    ) -> AppResult<bool> {
        let binding = stored_password.context("No password")?;
        let stored_password = PasswordHash::new(binding.as_str())
            .context("Failed to parse hash in PHC string format.")?;

        Argon2::default()
            .verify_password(attempted_pasword.as_bytes(), &stored_password)
            .context("Invalid password.")
            .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;
        Ok(true)
    }
}
