use crate::domain::*;

pub struct App {
    id: Id,
    name: String,
}

impl App {
    pub fn new(name: String) -> Self {
        let id = Id::new(Kind::App);
        App { id, name }
    }
}
