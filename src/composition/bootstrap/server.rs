use axum::{Router, routing::get};
use tokio::net::TcpListener;

pub struct Server;

impl Server {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Starts the HTTP server and blocks until it shuts down.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The TCP listener cannot be bound
    /// - The server fails while serving requests
    pub async fn run(&self) -> Result<(), std::io::Error> {
        let listener = Self::setup_listener().await?;
        let router = Self::setup_router();

        println!("🚀 Server started at http://{}", listener.local_addr()?);

        Self::setup_axum(listener, router).await?;

        Ok(())
    }

    async fn setup_listener() -> std::io::Result<TcpListener> {
        TcpListener::bind("0.0.0.0:3000").await
    }

    fn setup_router() -> Router {
        Router::new().route("/", get(|| async { "Hello, world!" }))
    }

    async fn setup_axum(listener: TcpListener, router: Router) -> std::io::Result<()> {
        axum::serve(listener, router).await
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
