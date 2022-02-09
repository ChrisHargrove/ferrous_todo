use dirs;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use crate::models::TodoList;

pub fn get_todo_data_path() -> PathBuf {
    let mut data_directory =
        dirs::data_local_dir().expect("Could not find the application directory");

    data_directory.push("ferrous_todo");
    fs::create_dir_all(data_directory.clone())
        .expect(format!("Path: {:?}", data_directory).as_str());
    data_directory.push("todo_list.json");
    data_directory
}

pub fn open_or_create_todo_file(data_path: PathBuf) -> File {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(data_path.clone())
        .expect(format!("Path: {:?}", data_path).as_str()); //TODO: comeback and handle this gracefully!
    file
}

pub fn deserialize_todo_list() -> TodoList {
    let file = open_or_create_todo_file(get_todo_data_path());
    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(todo_list) => return todo_list,
        Err(_) => {
            println!("Couldn't deserialize TodoList creating a new one");
            return TodoList::new();
        }
    };
}

pub fn serialize_todo_list(todo_list: TodoList) {
    let file = open_or_create_todo_file(get_todo_data_path());
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &todo_list).expect("Failed to write TodoList to file!");
}
