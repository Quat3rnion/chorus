pub use apierror::*;
pub use auth::*;
pub use channel::*;
pub use guild::*;
pub use message::*;
pub use relationship::*;
pub use role::*;
pub use user::*;

mod apierror;
mod auth;
mod channel;
mod guild;
mod message;
mod relationship;
mod role;
mod user;

#[cfg(test)]
mod schemas_tests {
    use crate::errors::FieldFormatError;

    use super::*;

    #[test]
    fn password_too_short() {
        assert_eq!(
            AuthPassword::new("".to_string()),
            Err(FieldFormatError::PasswordError)
        );
    }

    #[test]
    fn password_too_long() {
        let mut long_pw = String::new();
        for _ in 0..73 {
            long_pw += "a";
        }
        assert_eq!(
            AuthPassword::new(long_pw),
            Err(FieldFormatError::PasswordError)
        );
    }

    #[test]
    fn username_too_short() {
        assert_eq!(
            AuthUsername::new("T".to_string()),
            Err(FieldFormatError::UsernameError)
        );
    }

    #[test]
    fn username_too_long() {
        let mut long_un = String::new();
        for _ in 0..33 {
            long_un += "a";
        }
        assert_eq!(
            AuthUsername::new(long_un),
            Err(FieldFormatError::UsernameError)
        );
    }

    #[test]
    fn consent_false() {
        assert_eq!(
            RegisterSchema::builder("Test", false).build(),
            Err(FieldFormatError::ConsentError)
        );
    }

    #[test]
    fn invalid_email() {
        assert_eq!(
            AuthEmail::new("p@p.p".to_string()),
            Err(FieldFormatError::EmailError)
        )
    }

    #[test]
    fn valid_email() {
        let reg = RegisterSchemaOptions {
            email: Some("me@mail.de".to_string()),
            ..RegisterSchema::builder("Testy", true)
        }
        .build();
        assert_ne!(reg, Err(FieldFormatError::EmailError));
    }
}
