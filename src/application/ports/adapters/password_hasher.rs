pub trait PasswordHasherPort: Send + Sync {
    fn hash_password(&self, password: String) -> String;
}
