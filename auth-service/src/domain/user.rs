use crate::domain::{Email, Password};

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: Email,
    password: Password,
    requires_2fa: bool
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa
        }
    }

    pub fn password(&self) -> &Password {
        &self.password
    }

    pub fn requires_2fa(&self) -> bool {
        self.requires_2fa
    }
}
