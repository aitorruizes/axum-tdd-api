use std::sync::Arc;

use crate::{
    application::{
        inputs::auth::sign_up::SignUpInput,
        ports::{
            adapters::{
                id_generator::IdGeneratorPort, password_hasher::PasswordHasherPort, time::TimePort,
            },
            use_cases::auth::sign_up::SignUpPort,
        },
    },
    domain::{
        dtos::user::{CreateUserDto, FindUserByEmailDto},
        entities::user::UserEntity,
        errors::domain::DomainError,
        repositories::user::UserPersistencePort,
    },
};

pub struct SignUpUseCase {
    id_generator: Arc<dyn IdGeneratorPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    time: Arc<dyn TimePort>,
    repository: Arc<dyn UserPersistencePort>,
}

impl SignUpUseCase {
    pub const fn new(
        id_generator: Arc<dyn IdGeneratorPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
        time: Arc<dyn TimePort>,
        repository: Arc<dyn UserPersistencePort>,
    ) -> Self {
        Self {
            id_generator,
            password_hasher,
            time,
            repository,
        }
    }
}

#[async_trait::async_trait]
impl SignUpPort for SignUpUseCase {
    async fn perform(&self, input: SignUpInput) -> Result<UserEntity, DomainError> {
        if input.password != input.password_confirmation {
            return Err(DomainError::PasswordMismatch);
        }

        let find_user_by_email_dto = FindUserByEmailDto {
            email: input.email.clone(),
        };

        let found_user = self
            .repository
            .find_by_email(find_user_by_email_dto)
            .map_err(|err| DomainError::Internal(err.to_string()))?;

        if found_user.is_some() {
            return Err(DomainError::UserAlreadyExists);
        }

        let create_user_dto = CreateUserDto {
            id: self.id_generator.generate_id(),
            first_name: input.first_name,
            last_name: input.last_name,
            email: input.email,
            password_hash: self.password_hasher.hash_password(input.password),
            created_at: self.time.utc_now(),
        };

        let user_entity = match self.repository.create(create_user_dto) {
            Ok(entity) => entity,
            Err(err) => return Err(DomainError::Internal(err.to_string())),
        };

        Ok(user_entity)
    }
}

#[cfg(test)]
mod tests {
    use mockall::mock;
    use std::sync::Arc;

    use crate::{
        application::{
            inputs::auth::sign_up::SignUpInput,
            ports::{
                adapters::{
                    id_generator::IdGeneratorPort, password_hasher::PasswordHasherPort,
                    time::TimePort,
                },
                use_cases::auth::sign_up::SignUpPort,
            },
            use_cases::auth::sign_up::SignUpUseCase,
        },
        domain::{
            dtos::user::{CreateUserDto, FindUserByEmailDto},
            entities::user::UserEntity,
            errors::domain::DomainError,
            repositories::user::UserPersistencePort,
        },
    };

    mock! {
        pub IdGeneratorPort {}

        impl IdGeneratorPort for IdGeneratorPort {
            fn generate_id(&self) -> String;
        }
    }

    mock! {
        pub PasswordHasherPort {}

        impl PasswordHasherPort for PasswordHasherPort {
            fn hash_password(&self, password: String) -> String;
        }
    }

    mock! {
        pub TimePort {}

        impl TimePort for TimePort {
            fn utc_now(&self) -> i64;
        }
    }

    mock! {
        pub UserPersistencePort {}

        impl UserPersistencePort for UserPersistencePort {
            fn create(&self, dto: CreateUserDto) -> Result<UserEntity, DomainError>;
            fn find_by_email(&self, dto: FindUserByEmailDto) -> Result<Option<UserEntity>, DomainError>;
        }
    }

    #[tokio::test]
    async fn should_successfully_sign_up_user() {
        let mut id_generator = MockIdGeneratorPort::default();

        id_generator
            .expect_generate_id()
            .times(1)
            .returning(|| "generated_id".to_string());

        let mut password_hasher = MockPasswordHasherPort::default();

        password_hasher
            .expect_hash_password()
            .times(1)
            .returning(|_| "password_hash".to_string());

        let mut time = MockTimePort::default();

        time.expect_utc_now().times(1).returning(|| 1_000_000);

        let mut repository = MockUserPersistencePort::default();

        repository
            .expect_find_by_email()
            .times(1)
            .returning(|_| Ok(None));

        repository.expect_create().times(1).returning(|_| {
            Ok(UserEntity::new(
                "John".to_string(),
                "Doe".to_string(),
                "john.doe@mail.com".to_string(),
                "SuperSecret123".to_string(),
                1_000_000,
                1_000_000,
            ))
        });

        let use_case = SignUpUseCase::new(
            Arc::new(id_generator),
            Arc::new(password_hasher),
            Arc::new(time),
            Arc::new(repository),
        );

        let input = SignUpInput {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@mail.com".to_string(),
            password: "SuperSecret123".to_string(),
            password_confirmation: "SuperSecret123".to_string(),
        };

        let result = use_case.perform(input).await;

        assert!(result.is_ok());

        let user_entity = result.unwrap();

        assert_eq!(
            user_entity,
            UserEntity::new(
                "John".to_string(),
                "Doe".to_string(),
                "john.doe@mail.com".to_string(),
                "SuperSecret123".to_string(),
                1_000_000,
                1_000_000,
            )
        );
    }

    #[tokio::test]
    async fn should_return_error_if_password_mismatch() {
        let id_generator = MockIdGeneratorPort::default();
        let password_hasher = MockPasswordHasherPort::default();
        let time = MockTimePort::default();
        let repository = MockUserPersistencePort::default();

        let use_case = SignUpUseCase::new(
            Arc::new(id_generator),
            Arc::new(password_hasher),
            Arc::new(time),
            Arc::new(repository),
        );

        let input = SignUpInput {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@mail.com".to_string(),
            password: "SuperSecret123".to_string(),
            password_confirmation: "SuperSecret1234".to_string(),
        };

        let result = use_case.perform(input).await;

        assert!(result.is_err());

        let result_err = result.unwrap_err();

        assert_eq!(result_err, DomainError::PasswordMismatch);
    }

    #[tokio::test]
    async fn should_return_error_if_user_already_exists() {
        let id_generator = MockIdGeneratorPort::default();
        let password_hasher = MockPasswordHasherPort::default();
        let time = MockTimePort::default();

        let mut repository = MockUserPersistencePort::default();

        repository.expect_find_by_email().times(1).returning(|_| {
            Ok(Some(UserEntity::new(
                "John".to_string(),
                "Doe".to_string(),
                "john.doe@mail.com".to_string(),
                "SuperSecret123".to_string(),
                1_000_000,
                1_000_000,
            )))
        });

        let use_case = SignUpUseCase::new(
            Arc::new(id_generator),
            Arc::new(password_hasher),
            Arc::new(time),
            Arc::new(repository),
        );

        let input = SignUpInput {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@mail.com".to_string(),
            password: "SuperSecret123".to_string(),
            password_confirmation: "SuperSecret123".to_string(),
        };

        let result = use_case.perform(input).await;

        assert!(result.is_err());

        let result_err = result.unwrap_err();

        assert_eq!(result_err, DomainError::UserAlreadyExists);
    }

    #[tokio::test]
    async fn should_return_error_if_find_by_email_fails() {
        let id_generator = MockIdGeneratorPort::default();
        let password_hasher = MockPasswordHasherPort::default();
        let time = MockTimePort::default();
        let mut repository = MockUserPersistencePort::default();

        repository
            .expect_find_by_email()
            .times(1)
            .returning(|_| Err(DomainError::Internal("Find by e-mail failed".to_string())));

        let use_case = SignUpUseCase::new(
            Arc::new(id_generator),
            Arc::new(password_hasher),
            Arc::new(time),
            Arc::new(repository),
        );

        let input = SignUpInput {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@mail.com".to_string(),
            password: "SuperSecret123".to_string(),
            password_confirmation: "SuperSecret123".to_string(),
        };

        let result = use_case.perform(input).await;

        assert!(result.is_err());

        let result_err = result.unwrap_err();

        assert_eq!(
            result_err,
            DomainError::Internal("Something went wrong: Find by e-mail failed".to_string())
        );
    }

    #[tokio::test]
    async fn should_return_error_if_create_fails() {
        let mut id_generator = MockIdGeneratorPort::default();

        id_generator
            .expect_generate_id()
            .times(1)
            .returning(|| "generated_id".to_string());

        let mut password_hasher = MockPasswordHasherPort::default();

        password_hasher
            .expect_hash_password()
            .times(1)
            .returning(|_| "password_hash".to_string());

        let mut time = MockTimePort::default();

        time.expect_utc_now().times(1).returning(|| 1_000_000);

        let mut repository = MockUserPersistencePort::default();

        repository
            .expect_find_by_email()
            .times(1)
            .returning(|_| Ok(None));

        repository
            .expect_create()
            .times(1)
            .returning(|_| Err(DomainError::Internal("Create failed".to_string())));

        let use_case = SignUpUseCase::new(
            Arc::new(id_generator),
            Arc::new(password_hasher),
            Arc::new(time),
            Arc::new(repository),
        );

        let input = SignUpInput {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@mail.com".to_string(),
            password: "SuperSecret123".to_string(),
            password_confirmation: "SuperSecret123".to_string(),
        };

        let result = use_case.perform(input).await;

        assert!(result.is_err());

        let result_err = result.unwrap_err();

        assert_eq!(
            result_err,
            DomainError::Internal("Something went wrong: Create failed".to_string())
        );
    }
}
