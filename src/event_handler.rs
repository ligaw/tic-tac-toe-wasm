extern crate serde_json;
use serde_json::Value;
use crate::GameWrapper::make_move();
use std::collections::HashMap;

#[derive(Deserialize)]
struct User {
    id: i32,
}

#[derive(Deserialize)]
struct Contact {
    name: String,
    phone: String,
}

enum Command {
    MakeMove,
}

impl Command {
    fn from_str(command: &str) -> Option<Command> {
        match command {
            "make-move" => Some(Command::MakeMove),
            _ => None,
        }
    }
}

#[no_mangle]
pub fn event_handler(command: &str, arg: &str) {
    match Command::from_str(command) {
        Some(Command::DeleteUser) => {
            let user: User = serde_json::from_str(arg).unwrap();
            delete_user(user);
        },
        None => println!("Unknown command"),
    }
}

#[test]
fn test_command_from_str() {
    assert_eq!(Command::from_str("make-move"), Some(Command::MakeMove));
    assert_eq!(Command::from_str("unknown"), None);
}

#[test]
fn test_event_handler() {
    event_handler("make-move", [1,1]);
    assert_eq!(make_move([1, 2]), );
}
