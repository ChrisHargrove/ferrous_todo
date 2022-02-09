mod cli;
mod io;

mod commands;
mod models;

use cli::{Args, Parser};

fn main() {
    let args = Args::parse();

    let mut todo_list = io::deserialize_todo_list();
    let commander = commands::Commander::new();

    commander.command(args.command, &mut todo_list);

    io::serialize_todo_list(todo_list);
}
