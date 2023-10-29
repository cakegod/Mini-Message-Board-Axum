use askama::Template;
use axum::{
    routing::{get, post},
    Form, Router,
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Template)]
#[template(path = "hello.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    messages: Vec<Message>,
}

#[derive(Template)]
#[template(path = "new.html")]
struct NewTemplate<'a> {
    title: &'a str,
}

#[derive(Serialize, Debug)]
struct Message {
    text: String,
    user: String,
    #[serde(serialize_with = "serialize_date")]
    added: DateTime<Local>,
}

fn serialize_date<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.to_rfc2822())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/new", get(new))
        .route("/new", post(newish));
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
            added: Default::default(),
        },
        Message {
            text: "Hello World!".to_string(),
            user: "Charles".to_string(),
            added: Default::default(),
        },
    ];

    IndexTemplate {
        title: "Mini Messageboard",
        messages,
    }
}

async fn new() -> NewTemplate<'static> {
    NewTemplate { title: "new" }
}

#[derive(Deserialize)]
struct MessageForm {
    user: String,
    text: String,
}

async fn newish(Form(MessageForm { text, user }): Form<MessageForm>) {
    let msg = Message {
        text,
        user,
        added: Local::now(),
    };
    println!("{:?}", msg);
}
