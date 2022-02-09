use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: uuid::Uuid,
    title: String,
    message: Option<String>,
    urgent: bool,
    complete: bool,
}

impl Todo {
    pub fn new(title: String, message: Option<String>, urgent: bool) -> Self {
        Todo {
            id: uuid::Uuid::new_v4(),
            title: title,
            message: message,
            urgent: urgent,
            complete: false,
        }
    }

    pub fn modify_title(&mut self, title: String) -> &Self {
        self.title = title;
        self
    }

    pub fn modify_message(&mut self, message: String) -> &Self {
        self.message = Some(message);
        self
    }

    pub fn modify_urgent(&mut self, urgent: bool) -> &Self {
        self.urgent = urgent;
        self
    }

    pub fn modify_completion(&mut self, completed: bool) -> &Self {
        self.complete = completed;
        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { list: Vec::new() }
    }
}
