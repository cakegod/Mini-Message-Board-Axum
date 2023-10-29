use crate::handlers::Message;
use askama::Template;

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
