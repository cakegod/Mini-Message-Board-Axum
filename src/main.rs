use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{
    Router,
    routing::{get, post},
};

mod handlers;
mod structs;
mod templates;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_state = structs::AppState {
        messages: Arc::new(RwLock::new(vec![
            structs::Message {
                text: "Hi there!".to_string(),
                user: "Amando".to_string(),
                added: Default::default(),
            },
            structs::Message {
                text: "Hello World!".to_string(),
                user: "Charles".to_string(),
                added: Default::default(),
            },
        ])),
    };

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/new", get(handlers::new))
        .route("/new", post(handlers::submit_new))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
