use axum::{extract::State, Form, response::Redirect};
use chrono::Local;

use crate::structs::{AppState, Message, MessageForm};
use crate::templates;

pub async fn index(State(app_state): State<AppState>) -> templates::Index<'static> {
    templates::Index {
        title: "Mini Message Board",
        messages: app_state.messages(),
    }
}

pub async fn new() -> templates::New<'static> {
    templates::New { title: "new" }
}

pub async fn submit_new(
    State(app_state): State<AppState>,
    Form(MessageForm { text, user }): Form<MessageForm>,
) -> Redirect {
    app_state.add_message(Message {
        text,
        user,
        added: Local::now(),
    });
    Redirect::to("/")
}
