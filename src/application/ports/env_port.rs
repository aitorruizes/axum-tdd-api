use std::str::FromStr;

#[derive(Debug)]
pub enum EnvError {
    EnvNotInitialized,
    FileNotFound,
    FileNotLoaded,
    MissingVariables(Vec<String>),
    VariableNotSet(&'static str),
    VariableParsing {
        key: &'static str,
        value: String,
        parsing_type: &'static str,
    },
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvNotInitialized => write!(
                f,
                "Environment not initialized. Ensure the env adapter is set up before use."
            ),
            Self::FileNotFound => write!(f, ".env file not found"),
            Self::FileNotLoaded => write!(f, ".env file not loaded"),
            Self::MissingVariables(variables) => {
                if variables.len() > 1 {
                    write!(f, "{} are not set in .env file", variables.join(", "))
                } else {
                    write!(f, "{} is not set in .env file", variables.join(" "))
                }
            }
            Self::VariableNotSet(key) => write!(f, "env variable '{key}' not set"),
            Self::VariableParsing {
                key,
                value,
                parsing_type,
            } => write!(
                f,
                "Could not parse env variable '{key}' with value '{value}' to '{parsing_type}'"
            ),
        }
    }
}

impl std::error::Error for EnvError {}

pub trait EnvPort {
    /// Loads the environment variables from the `.env` file.
    ///
    /// # Errors
    ///
    /// Returns an [`EnvError::FileNotFound`] if the `.env` file cannot be found.
    fn load_env_file(&mut self) -> Result<(), EnvError>;

    /// Checks that all required environment variables are set and valid.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `.env` file has not been loaded (`EnvError::FileNotLoaded`)
    /// - Any required environment variable is missing (`EnvError::MissingVariables`)
    fn check_env_vars(&self) -> Result<(), EnvError>;

    /// Retrieves an environment variable and attempts to parse it to the specified type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to parse the environment variable into. Must implement [`FromStr`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `.env` file has not been loaded (`EnvError::FileNotLoaded`)
    /// - The variable is not set (`EnvError::VariableNotSet`)
    /// - The value cannot be parsed into type `T` (`EnvError::VariableParsing`)
    fn get_env_var<T: FromStr>(&self, key: &'static str) -> Result<T, EnvError>;

    /// Retrieves the `SERVER_HOST` environment variable.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `.env` file has not been loaded
    /// - The `SERVER_HOST` variable is missing or cannot be parsed as a string
    fn get_server_host(&self) -> Result<String, EnvError>;

    /// Retrieves the `SERVER_PORT` environment variable.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The `.env` file has not been loaded
    /// - The `SERVER_PORT` variable is missing or cannot be parsed as a `u16`
    fn get_server_port(&self) -> Result<u16, EnvError>;
}
