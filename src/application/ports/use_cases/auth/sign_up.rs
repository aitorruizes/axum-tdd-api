use crate::{
    application::inputs::auth::sign_up::SignUpInput,
    domain::{entities::user::UserEntity, errors::domain::DomainError},
};

#[async_trait::async_trait]
pub trait SignUpPort {
    async fn perform(&self, input: SignUpInput) -> Result<UserEntity, DomainError>;
}
