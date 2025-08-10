#[derive(Clone, Debug, PartialEq)]
pub struct Password(String);

#[derive(Debug, PartialEq)]
pub enum PasswordParseError {
    TooShort,
    TooLong,
    Empty,
    MissingUppercase,
    MissingLowercase,
    MissingDigit,
    MissingSpecialChar,
}

impl Password {
    pub fn parse(password: String) -> Result<Password, PasswordParseError> {
        if password.trim().is_empty() {
            return Err(PasswordParseError::Empty);
        }

        if password.len() < 8 {
            return Err(PasswordParseError::TooShort);
        }

        if password.len() > 128 {
            return Err(PasswordParseError::TooLong);
        }

        if !password.chars().any(|c| c.is_ascii_uppercase()) {
            return Err(PasswordParseError::MissingUppercase);
        }

        if !password.chars().any(|c| c.is_ascii_lowercase()) {
            return Err(PasswordParseError::MissingLowercase);
        }

        if !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(PasswordParseError::MissingDigit);
        }

        if !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
            return Err(PasswordParseError::MissingSpecialChar);
        }

        Ok(Password(password))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let result = Password::parse("Password123!".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, "Password123!");
    }

    #[test]
    fn test_empty_password() {
        let result = Password::parse("".to_string());
        assert_eq!(result, Err(PasswordParseError::Empty));
    }

    #[test]
    fn test_whitespace_only_password() {
        let result = Password::parse("   ".to_string());
        assert_eq!(result, Err(PasswordParseError::Empty));
    }

    #[test]
    fn test_password_too_short() {
        let result = Password::parse("Pass1!".to_string());
        assert_eq!(result, Err(PasswordParseError::TooShort));
    }

    #[test]
    fn test_password_too_long() {
        let long_password = "P".repeat(120) + "assword1!";
        let result = Password::parse(long_password);
        assert_eq!(result, Err(PasswordParseError::TooLong));
    }

    #[test]
    fn test_password_missing_uppercase() {
        let result = Password::parse("password123!".to_string());
        assert_eq!(result, Err(PasswordParseError::MissingUppercase));
    }

    #[test]
    fn test_password_missing_lowercase() {
        let result = Password::parse("PASSWORD123!".to_string());
        assert_eq!(result, Err(PasswordParseError::MissingLowercase));
    }

    #[test]
    fn test_password_missing_digit() {
        let result = Password::parse("Password!".to_string());
        assert_eq!(result, Err(PasswordParseError::MissingDigit));
    }

    #[test]
    fn test_password_missing_special_char() {
        let result = Password::parse("Password123".to_string());
        assert_eq!(result, Err(PasswordParseError::MissingSpecialChar));
    }
}