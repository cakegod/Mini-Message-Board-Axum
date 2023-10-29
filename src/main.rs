mod handlers;
mod templates;

use axum::{
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Local};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

fn serialize_date<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.to_rfc2822())
}

#[derive(Clone)]
pub struct AppState {
    messages: Arc<RwLock<Vec<handlers::Message>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_state = AppState {
        messages: Arc::new(RwLock::new(vec![
            handlers::Message {
                text: "Hi there!".to_string(),
                user: "Amando".to_string(),
                added: Default::default(),
            },
            handlers::Message {
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
