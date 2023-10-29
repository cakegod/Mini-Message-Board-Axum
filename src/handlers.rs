use crate::serialize_date;
use axum::{extract::State, response::Redirect, Form};
use chrono::{DateTime, Local};
use serde::Deserialize;

use crate::{templates, AppState};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Message {
    pub text: String,
    pub user: String,
    #[serde(serialize_with = "serialize_date")]
    pub added: DateTime<Local>,
}

#[derive(Deserialize)]
pub struct MessageForm {
    user: String,
    text: String,
}

pub async fn index(State(app_state): State<AppState>) -> templates::Index<'static> {
    let reader = app_state.messages.read().unwrap();
    templates::Index {
        title: "Mini Messageboard",
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
