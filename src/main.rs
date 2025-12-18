use crate::composition::bootstrap::server::Server;

pub mod composition {
    pub mod bootstrap {
        pub mod server;
    }
}

#[tokio::main]
async fn main() {
    let server = Server;

    if let Err(err) = server.run().await {
        eprintln!("Could not run server: {err}")
    }
}
