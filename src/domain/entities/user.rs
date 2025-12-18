#[derive(Debug, PartialEq, Eq)]
pub struct UserEntity {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl UserEntity {
    pub const fn new(
        id: String,
        first_name: String,
        last_name: String,
        email: String,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            email,
            created_at,
            updated_at,
        }
    }
}
