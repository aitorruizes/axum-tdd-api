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
        pub mod adapters {
            pub mod env;
            pub mod id_generator;
            pub mod password_hasher;
            pub mod time;
        }

        pub mod use_cases {
            pub mod auth {
                pub mod sign_up;
            }
        }
    }

    pub mod inputs {
        pub mod auth {
            pub mod sign_up;
        }
    }

    pub mod use_cases {
        pub mod auth {
            pub mod sign_up;
        }
    }
}

pub mod infrastructure {
    pub mod adapters {
        pub mod dotenvy;
    }
}

pub mod domain {
    pub mod entities {
        pub mod user;
    }

    pub mod errors {
        pub mod domain;
    }

    pub mod repositories {
        pub mod user;
    }

    pub mod dtos {
        pub mod user;
    }
}

#[tokio::main]
async fn main() {
    let mut server = Server::new();

    if let Err(err) = server.run().await {
        eprintln!("Could not run server: {err}");
    }
}
