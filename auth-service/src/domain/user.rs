#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: String,
    password: String,
    requires_2fa: bool
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa
        }
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
