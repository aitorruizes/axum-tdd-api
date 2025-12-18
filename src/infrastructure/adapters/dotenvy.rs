use crate::application::ports::env_port::{EnvError, EnvPort};

pub struct DotenvyAdapter {
    pub is_env_file_loaded: bool,
}

impl DotenvyAdapter {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            is_env_file_loaded: false,
        }
    }
}

impl EnvPort for DotenvyAdapter {
    fn load_env_file(&mut self) -> Result<(), EnvError> {
        dotenvy::dotenv().map_err(|_| EnvError::FileNotFound)?;

        self.is_env_file_loaded = true;

        Ok(())
    }

    fn check_env_vars(&self) -> Result<(), EnvError> {
        if !self.is_env_file_loaded {
            return Err(EnvError::FileNotLoaded);
        }

        let mut variables = Vec::new();

        if self.get_env_var::<String>("SERVER_HOST").is_err() {
            variables.push("SERVER_HOST".to_string());
        }

        if self.get_env_var::<u16>("SERVER_PORT").is_err() {
            variables.push("SERVER_PORT".to_string());
        }

        if !variables.is_empty() {
            return Err(EnvError::MissingVariables(variables));
        }

        Ok(())
    }

    fn get_env_var<T: std::str::FromStr>(&self, key: &'static str) -> Result<T, EnvError> {
        if !self.is_env_file_loaded {
            return Err(EnvError::FileNotLoaded);
        }

        let variable = std::env::var(key).map_err(|_| EnvError::VariableNotSet(key))?;

        variable
            .parse::<T>()
            .map_err(|_| EnvError::VariableParsing {
                key,
                value: variable,
                parsing_type: std::any::type_name::<T>(),
            })
    }

    fn get_server_host(&self) -> Result<String, EnvError> {
        self.get_env_var("SERVER_HOST")
    }

    fn get_server_port(&self) -> Result<u16, EnvError> {
        self.get_env_var("SERVER_PORT")
    }
}

impl Default for DotenvyAdapter {
    fn default() -> Self {
        Self::new()
    }
}
