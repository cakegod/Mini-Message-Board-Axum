use axum::{extract::State, Form, response::Redirect};
use chrono::Local;

use crate::structs::{AppState, Message, MessageForm};
use crate::templates;

pub async fn index(State(app_state): State<AppState>) -> templates::Index<'static> {
    let reader = app_state.messages.read().unwrap();
    templates::Index {
        title: "Mini Message Board",
        messages: reader.to_vec(),
    }
}

pub async fn new() -> templates::New<'static> {
    templates::New { title: "new" }
}

pub async fn submit_new(
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
