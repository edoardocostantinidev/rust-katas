use crate::context::Context;
use crate::model::Todo;
use cli_table::{print_stdout, Cell, Style, Table};
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
                let mut new_context: Context = Context::new();
                let no_args_message =
                    "todo command requires an argument such as (create, retrieve, update, delete)";
                if args.len() == 0 {
                    println!("{}", no_args_message);
                    return Ok(context);
                }
                match args[0].as_str() {
                    "create" => {
                        if args.len() < 2 {
                            return Err("todo create {} <- title missing".to_string());
                        }
                        let mut todos = context.todos;
                        let id = context.last_id + 1;
                        let title = String::from(args[1].clone());
                        let mut todo = Todo {
                            title: title,
                            completed: false,
                            checklist: HashMap::new(),
                        };

                        if args.len() > 2 {
                            let mut checklist = HashMap::new();
                            for i in 2..args.len() {
                                let item = String::from(args[i].clone());
                                checklist.insert(item, false);
                            }
                            todo.checklist = checklist;
                        }

                        todos.insert(id, todo);
                        new_context.last_id = id;
                        new_context.todos = todos;
                        println!("todo created with id: {}", id);
                    }
                    "retrieve" => {
                        if args.len() == 1 {
                            let todo_cells = context
                                .todos
                                .iter()
                                .map(|(k, v)| vec![k.cell(), v.clone().title.cell()]);
                            let table = todo_cells
                                .table()
                                .title(vec!["ID".cell().bold(true), "Title".cell().bold(true)]);
                            print_stdout(table).unwrap();
                        }

                        if args.len() == 2 {
                            let id = args[1].parse::<i32>().unwrap();
                            let todo = context.todos.get(&id);
                            if todo.is_none() {
                                return Err(format!("todo with id {} not found", id));
                            }
                            let todo = todo.unwrap();
                            let checklist = todo.checklist.clone();
                            let checklist_cells =
                                checklist.iter().map(|(k, v)| vec![k.cell(), v.cell()]);
                            let table = checklist_cells.table().title(vec![
                                "Checklist Item".cell().bold(true),
                                "Completed".cell().bold(true),
                            ]);
                            print_stdout(table).unwrap();
                        }
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
                Ok(new_context)
            },
        )
    }
}

//tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_help() {
        let command = CommandBuilder::help();
        assert_eq!(command.name, "help");
    }
    #[test]
    fn test_todo() {
        let command = CommandBuilder::todo();
        assert_eq!(command.name, "todo");
        let context: Context = Context::new();
        let new_context = (command.handler)(
            context,
            vec![
                "create".to_string(),
                "test1".to_string(),
                "checklist1".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(new_context.todos.len(), 1);
        assert_eq!(new_context.todos.get(&1).unwrap().title, "test1");
        assert_eq!(new_context.todos.get(&1).unwrap().checklist.len(), 1);
    }
}
