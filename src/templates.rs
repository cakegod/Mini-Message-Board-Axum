use askama::Template;

use crate::structs::Message;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct Index<'a> {
    pub title: &'a str,
    pub messages: Vec<Message>,
}

#[derive(Template)]
#[template(path = "new.html")]
pub struct New<'a> {
    pub title: &'a str,
}
