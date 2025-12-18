use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::{
    application::ports::adapters::env::{EnvError, EnvPort},
    infrastructure::adapters::dotenvy::DotenvyAdapter,
};

pub struct Server {
    env_adapter: Option<DotenvyAdapter>,
}

impl Server {
    #[must_use]
    pub const fn new() -> Self {
        Self { env_adapter: None }
    }

    /// Starts the HTTP server and blocks until it shuts down.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The TCP listener cannot be bound
    /// - The server fails while serving requests
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.setup_env()?;

        let listener = self.setup_listener().await?;
        let router = Self::setup_router();

        println!("🚀 Server started at http://{}", listener.local_addr()?);

        Self::setup_axum(listener, router).await?;

        Ok(())
    }

    async fn setup_listener(&self) -> Result<TcpListener, Box<dyn std::error::Error>> {
        let env_adapter = self
            .env_adapter
            .as_ref()
            .ok_or(EnvError::EnvNotInitialized)?;

        let server_host: String = env_adapter.get_env_var("SERVER_HOST")?;
        let server_port: u16 = env_adapter.get_env_var("SERVER_PORT")?;
        let server_address = format!("{server_host}:{server_port}");

        Ok(TcpListener::bind(server_address).await?)
    }

    fn setup_router() -> Router {
        Router::new().route("/", get(|| async { "Hello, world!" }))
    }

    async fn setup_axum(listener: TcpListener, router: Router) -> std::io::Result<()> {
        axum::serve(listener, router).await
    }

    fn setup_env(&mut self) -> Result<(), EnvError> {
        let mut adapter = DotenvyAdapter::new();

        adapter.load_env_file()?;
        adapter.check_env_vars()?;

        self.env_adapter = Some(adapter);

        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
