use crate::command::exit::Exit;
use crate::command::help::Help;
use crate::command::todo::Todo;
use crate::command::Command;
use crate::service::TodoService;

use std::io;
pub fn run(todo_service: TodoService) {
    greet();
    loop {
        let input = get_input();
        process_input(input, todo_service.clone());
    }
}
pub fn greet() {
    println!("TODO App 🚀 \nWrite a command (type help for... help?)👉");
}

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

pub fn process_input(input: String, todo_service: TodoService) {
    let input = input.trim();
    let input_split = input.split(" ").collect::<Vec<&str>>();
    let command = input_split[0];
    let args = &input_split[1..];
    println!("<===========TODO===========>");
    match command {
        "help" => Help {}.run(args),

        "todo" => Todo::new(todo_service).run(args),
        "exit" => Exit {}.run(args),
        _ => println!("Command not found"),
    }
}
