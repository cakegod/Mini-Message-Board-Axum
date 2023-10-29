use askama::Template;
use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Template)]
#[template(path = "hello.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    name: &'a str,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    text: String,
    user: String,
    added: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/new", get(message));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> IndexTemplate<'static> {
    let messages: Vec<Message> = vec![
        Message {
            text: "Hi there!".to_string(),
            user: "Amando".to_string(),
            added: "new Date()".to_string(),
        },
        Message {
            text: "Hello World!".to_string(),
            user: "Charles".to_string(),
            added: "new Date()".to_string(),
        },
    ];

    IndexTemplate {
        title: "Mini Messageboard",
        name: "hello world!",
        messages,
    }
}

async fn message() -> Json<Message> {
    // insert your application logic here
    Json(Message {
        text: "".to_string(),
        user: "".to_string(),
        added: "".to_string(),
    })
}
