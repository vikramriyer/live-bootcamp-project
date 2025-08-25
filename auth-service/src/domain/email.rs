#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Email(String);

#[derive(Debug, PartialEq)]
pub enum EmailParseError {
    InvalidFormat,
    Empty,
}

impl Email {
    pub fn parse(email: String) -> Result<Email, EmailParseError> {
        if email.trim().is_empty() {
            return Err(EmailParseError::Empty);
        }

        if !email.contains('@') || !email.contains('.') {
            return Err(EmailParseError::InvalidFormat);
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(EmailParseError::InvalidFormat);
        }

        if !parts[1].contains('.') {
            return Err(EmailParseError::InvalidFormat);
        }

        Ok(Email(email))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let result = Email::parse("test@example.com".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, "test@example.com");
    }

    #[test]
    fn test_empty_email() {
        let result = Email::parse("".to_string());
        assert_eq!(result, Err(EmailParseError::Empty));
    }

    #[test]
    fn test_whitespace_only_email() {
        let result = Email::parse("   ".to_string());
        assert_eq!(result, Err(EmailParseError::Empty));
    }

    #[test]
    fn test_email_without_at_symbol() {
        let result = Email::parse("testexample.com".to_string());
        assert_eq!(result, Err(EmailParseError::InvalidFormat));
    }

    #[test]
    fn test_email_without_dot() {
        let result = Email::parse("test@example".to_string());
        assert_eq!(result, Err(EmailParseError::InvalidFormat));
    }

    #[test]
    fn test_email_with_empty_local_part() {
        let result = Email::parse("@example.com".to_string());
        assert_eq!(result, Err(EmailParseError::InvalidFormat));
    }

    #[test]
    fn test_email_with_empty_domain() {
        let result = Email::parse("test@".to_string());
        assert_eq!(result, Err(EmailParseError::InvalidFormat));
    }

    #[test]
    fn test_email_with_multiple_at_symbols() {
        let result = Email::parse("test@example@com".to_string());
        assert_eq!(result, Err(EmailParseError::InvalidFormat));
    }

    #[test]
    fn test_email_domain_without_dot() {
        let result = Email::parse("test@example".to_string());
        assert_eq!(result, Err(EmailParseError::InvalidFormat));
    }
}