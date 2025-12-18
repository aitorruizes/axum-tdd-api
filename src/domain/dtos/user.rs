pub struct CreateUserDto {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: i64,
}

pub struct FindUserByEmailDto {
    pub email: String,
}
