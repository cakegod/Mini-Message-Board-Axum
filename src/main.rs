use askama::Template;
use axum::{
    extract::State,
    response::Redirect,
    routing::{get, post},
    Form, Router,
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

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

#[derive(Serialize, Debug, Clone)]
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

#[derive(Clone)]
struct AppState {
    messages: Arc<RwLock<Vec<Message>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_state = AppState {
        messages: Arc::new(RwLock::new(vec![
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
        ])),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/new", get(new))
        .route("/new", post(newish))
        .with_state(app_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(State(app_state): State<AppState>) -> IndexTemplate<'static> {
    let reader = app_state.messages.read().unwrap();
    IndexTemplate {
        title: "Mini Messageboard",
        messages: reader.to_vec(),
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

async fn newish(
    State(app_state): State<AppState>,
    Form(MessageForm { text, user }): Form<MessageForm>,
) -> Redirect {
    let mut writer = app_state.messages.write().unwrap();
    writer.push(Message {
        text,
        user,
        added: Local::now(),
    });
    Redirect::to("/")
}
