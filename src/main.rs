mod cli;
mod io;

mod commands;
mod models;

use cli::{Args, Command, Parser};

fn main() {
    let args = Args::parse();

    let todo_list = io::deserialize_todo_list();

    let _ = match args.command {
        Command::New {
            title,
            message,
            mark_urgent,
        } => 1,
        Command::Delete { id, all, urgent } => 2,
        Command::Update {
            id,
            title,
            message,
            mark_urgent,
        } => 3,
        Command::Show { id } => 4,
        Command::List { count, show_urgent } => 5,
    };

    io::serialize_todo_list(todo_list);
}
