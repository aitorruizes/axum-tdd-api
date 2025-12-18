use crate::domain::{
    dtos::user::{CreateUserDto, FindUserByEmailDto},
    entities::user::UserEntity,
    errors::domain::DomainError,
};

#[async_trait::async_trait]
pub trait UserPersistencePort: Send + Sync {
    fn create(&self, dto: CreateUserDto) -> Result<UserEntity, DomainError>;
    fn find_by_email(&self, dto: FindUserByEmailDto) -> Result<Option<UserEntity>, DomainError>;
}
