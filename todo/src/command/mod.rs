use crate::context::Context;
use crate::model::Todo;
use std::collections::HashMap;
pub type FnPointer = fn(Context, Vec<String>) -> Result<Context, String>;

pub struct Command {
    pub name: String,
    pub description: String,
    pub args: Vec<String>,
    pub handler: FnPointer,
}

impl Command {
    fn new(name: &str, description: &str, handler: FnPointer) -> Command {
        Command {
            name: name.to_string(),
            description: description.to_string(),
            args: Vec::new(),
            handler: handler,
        }
    }
}

pub struct CommandBuilder;
impl CommandBuilder {
    pub fn help() -> Command {
        let description = "
        - help: shows this message.
        ";

        let handler: FnPointer = |context, _args| -> Result<Context, String> {
            println!("help message here");
            Ok(context)
        };
        Command::new("help", description, handler)
    }

    pub fn todo() -> Command {
        let description = "
        - todo:
            - create: 
            - retrieve: 
            - update:
            - delete:
            - help: shows this message.
        ";
        Command::new(
            "todo",
            description,
            |context, args| -> Result<Context, String> {
                let no_args_message =
                    "todo command requires an argument such as (create, retrieve, update, delete)";
                if args.len() == 0 {
                    println!("{}", no_args_message);
                    return Ok(context);
                }
                match args[0].as_str() {
                    "create" => {
                        let mut todos = context.todos;
                        let id = context.last_id + 1;
                        print!("Enter todo title: ");
                        let mut title = String::new();
                        std::io::stdin().read_line(&mut title).unwrap();

                        todos.insert(
                            id,
                            Todo {
                                title: title,
                                completed: false,
                                checklist: HashMap::new(),
                            },
                        );

                        println!("todo created with id: {}", 0);
                    }
                    "retrieve" => {
                        println!("retrieve todo");
                    }
                    "update" => {
                        println!("update todo");
                    }
                    "delete" => {
                        println!("delete todo");
                    }
                    "help" => {
                        println!("help message here");
                    }
                    _ => {
                        println!("{}", no_args_message);
                    }
                }
                Ok(context)
            },
        )
    }
}
