use validator::validate_length;

#[derive(sqlx::Type, Clone)]
#[sqlx(transparent)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn parse(s: String) -> Result<HashedPassword, String> {
        if validate_length(&s, Some(32), None, None) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid password.", s))
        }
    }
}

impl TryFrom<String> for HashedPassword {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        HashedPassword::parse(value)
    }
}

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<HashedPassword> for String {
    fn from(val: HashedPassword) -> Self {
        val.0
    }
}

impl std::fmt::Debug for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.pad("(Secret)")
    }
}

#[cfg(test)]
mod tests {
    use super::HashedPassword;
    use claims::assert_err;
    use fake::faker::internet::en::Password;
    use fake::Fake;
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;

    #[test]
    fn empty_string_is_rejected() {
        let password = "".to_string();
        assert_err!(HashedPassword::parse(password));
    }

    #[test]
    fn password_short_rejected() {
        let password = "blah".to_string();
        assert_err!(HashedPassword::parse(password));
    }

    #[derive(Debug, Clone)]
    struct ValidpasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidpasswordFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let password = Password(32..255).fake_with_rng(&mut rng);

            Self(password)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidpasswordFixture) -> bool {
        HashedPassword::parse(valid_password.0).is_ok()
    }
}
