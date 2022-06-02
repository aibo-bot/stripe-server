use axum::{
    Server,
    Router,
    routing::post
};
use std::{
    fs,
    sync::Arc,
    net::SocketAddr
};

mod models;
mod routes;
use models::{
    Config,
    State
};
use routes::receive_webhook;

#[tokio::main]
async fn main() {
    let config_str = fs::read_to_string("config.toml")
        .expect("Failed to read config.toml file");
    let config: Config = toml::from_str(config_str.as_str())
        .expect("Failed to parse from config.toml");

    let state = Arc::new(State {
        config: config.clone()
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let router = Router::new()
        .route("/stripe-post", post(|x| async {
            receive_webhook(x, state).await
        }));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to start stripe-server");
}
