#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use crate::composition::bootstrap::server::Server;

pub mod composition {
    pub mod bootstrap {
        pub mod server;
    }
}

pub mod application {
    pub mod ports {
        pub mod env_port;
    }
}

pub mod infrastructure {
    pub mod adapters {
        pub mod dotenvy;
    }
}

#[tokio::main]
async fn main() {
    let mut server = Server::new();

    if let Err(err) = server.run().await {
        eprintln!("Could not run server: {err}");
    }
}
