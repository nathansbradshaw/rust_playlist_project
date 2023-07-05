use secrecy::{ExposeSecret, Secret};
use validator::validate_length;

#[derive(Debug)]
pub struct UserPassword(Secret<String>);

impl UserPassword {
    pub fn parse(s: String) -> Result<UserPassword, String> {
        if validate_length(&s, Some(8), Some(255), None) {
            Ok(Self(Secret::new(s)))
        } else {
            Err(format!("{} is not a valid user password.", s))
        }
    }
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}

impl Into<Secret<String>> for UserPassword {
    fn into(self) -> Secret<String> {
        self.0.to_owned()
    }
}

impl std::fmt::Display for UserPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.expose_secret().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::UserPassword;
    use claims::assert_err;
    use fake::faker::internet::en::Password;
    use fake::Fake;
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;

    #[test]
    fn empty_string_is_rejected() {
        let password = "".to_string();
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn password_short_rejected() {
        let password = "blah".to_string();
        assert_err!(UserPassword::parse(password));
    }

    #[test]
    fn password_long_is_rejected() {
        let password = "k4yKgZfiHbox3riwUgxFpNAPfdByGjPoMp4hE2CEGGop9Ea8dGdLKx00iiw596Num1MBFQFny232Vchm7nskzqP3nXS966THJBUXo7QEOKxhAWuIlKopFpLi8HCFsgBUL0VJPfYTrMd2t625E8t2veOgAmPHb1kRCtreXwoLC1jY1roL59EZkUa5GCUX8eZrPw5rceKf1h3jNRZZS9NMts0qf6LQUOe7Jeg1dfkxuVRSiqagFkhoeKL3gPvymDki
        ".to_string();
        assert_err!(UserPassword::parse(password));
    }

    #[derive(Debug, Clone)]
    struct ValidpasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidpasswordFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let password = Password(8..255).fake_with_rng(&mut rng);

            Self(password)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidpasswordFixture) -> bool {
        UserPassword::parse(valid_password.0).is_ok()
    }
}
