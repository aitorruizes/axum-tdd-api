use axum::{Router, routing::get};
use tokio::net::TcpListener;

pub struct Server;

impl Server {
    pub const fn new() -> Self {
        Self
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = self.setup_listener().await?;
        let router = self.setup_router();

        println!("🚀 Server started at http://{}", listener.local_addr()?);

        self.setup_axum(listener, router).await?;

        Ok(())
    }

    async fn setup_listener(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind("0.0.0.0:3000").await
    }

    fn setup_router(&self) -> Router {
        Router::new().route("/", get(|| async { "Hello, world!" }))
    }

    async fn setup_axum(&self, listener: TcpListener, router: Router) -> std::io::Result<()> {
        axum::serve(listener, router).await
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
