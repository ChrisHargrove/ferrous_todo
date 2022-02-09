use std::collections::HashMap;

use crate::{
    cli::Command,
    models::{Todo, TodoList},
};

type CommandCallback = fn(Command, &mut TodoList);

pub struct Commander {
    commands: HashMap<String, CommandCallback>,
}

impl Commander {
    pub fn new() -> Self {
        Commander {
            commands: HashMap::from([(String::from("New"), create_new_todo as CommandCallback)]),
        }
    }

    pub fn command(&self, command: Command, todo_list: &mut TodoList) {
        if let Some(callback) = self.commands.get(&command.to_string()) {
            callback(command, todo_list);
        }
    }
}

fn create_new_todo(command: Command, todo_list: &mut TodoList) {
    match command {
        Command::New {
            title,
            message,
            mark_urgent,
        } => {
            todo_list.list.push(Todo::new(title, message, mark_urgent));
        }
        _ => return,
    };
}
