pub use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    ///Creates a new todo
    New {
        ///The title of the new todo
        title: String,
        ///The new message for the todo
        #[clap(short = 'm', long = "message")]
        message: Option<String>,
        ///Mark this post as urgent
        #[clap(short = 'u', long = "urgent")]
        mark_urgent: bool,
    },
    ///Deletes a todo by its ID
    Delete {
        ///Numeric ID representing the post you want to delete
        id: Option<i64>,
        ///Delete all todo's
        #[clap(short = 'a', long = "all")]
        all: bool,
        ///Only urgent
        #[clap(short = 'u', long = "urgent")]
        urgent: bool,
    },
    ///Updates a todo by its ID
    Update {
        ///Numeric ID representing the post you want to update
        id: i64,
        ///The new title for the todo
        title: Option<String>,
        ///The new message for the todo
        #[clap(short = 'm', long = "message")]
        message: Option<String>,
        ///Mark this post as urgent
        #[clap(short = 'u', long = "urgent")]
        mark_urgent: bool,
    },
    ///Shows a specific todo by id
    Show {
        ///Numeric ID representing the post you want to show
        id: i64,
    },
    ///Lists the todo's
    List {
        ///Count of how many todo's to show
        count: Option<i64>,
        ///Show only urgent todo's
        #[clap(short = 'u', long = "urgent")]
        show_urgent: bool,
    },
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::New { .. } => String::from("New"),
            Command::Delete { .. } => String::from("Delete"),
            Command::Update { .. } => String::from("Update"),
            Command::Show { .. } => String::from("Show"),
            Command::List { .. } => String::from("List"),
        }
    }
}
