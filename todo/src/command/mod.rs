use crate::context::Context;
use crate::model::Todo;
use cli_table::{print_stdout, Cell, Style, Table};
use std::collections::HashMap;

pub type FnPointer = fn(&mut Context, Vec<String>) -> Result<(), String>;

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

        let handler: FnPointer = |_context, _args| -> Result<(), String> {
            println!("help message here");
            Ok(())
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
        Command::new("todo", description, |context, args| -> Result<(), String> {
            let no_args_message =
                "todo command requires an argument such as (create, retrieve, update, delete)";
            if args.len() == 0 {
                println!("{}", no_args_message);
                return Ok(());
            }
            match args[0].as_str() {
                "create" => {
                    if args.len() < 2 {
                        return Err("todo create {} <- title missing".to_string());
                    }
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

                    context.todos.insert(id, todo);
                    context.last_id = id;
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
                    if args.len() == 1 {
                        return Err(
                            "todo update {} <- update field missing (e.g. status, checklist)"
                                .to_string(),
                        );
                    }
                    if args.len() == 2 {
                        return Err("todo update {} {} <- id missing".to_string());
                    }

                    let field = args[1].clone();
                    let id = &args[2].parse::<i32>().unwrap();
                    match field.as_str() {
                        "status" => {
                            if args.len() == 3 {
                                return Err(format!(
                                    "todo update {} {} {} <- value missing",
                                    field, id, ""
                                ));
                            }
                            let status = args[3].parse::<bool>().unwrap();
                            let mut todo = context.todos.get_mut(id).unwrap();
                            todo.completed = status;
                        }
                        "checklist" => {
                            if args.len() == 3 {
                                return Err(format!(
                                    "todo update {} {} {} <- checklist item id missing",
                                    field, id, ""
                                ));
                            }

                            let item_id = args[3].clone();
                            let todo = context.todos.get_mut(id).unwrap();
                            let mut checklist_to_update = todo.checklist.clone();
                            let item = checklist_to_update.get_mut(&item_id).unwrap().clone();
                            checklist_to_update.insert(item_id, !item);
                            todo.checklist = checklist_to_update;
                        }
                        _ => {
                            return Err(format!(
                                "todo update {} {} <- update field not found",
                                field, id
                            ));
                        }
                    }
                }
                "delete" => {
                    if args.len() == 1 {
                        return Err("todo delete {} <- id missing".to_string());
                    }
                    let id = args[1].parse::<i32>().unwrap();
                    context.todos.remove(&id);
                }
                "help" => {
                    println!("help message here");
                }
                _ => {
                    println!("{}", no_args_message);
                }
            }
            Ok(())
        })
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
    fn test_todo_create() {
        let command = CommandBuilder::todo();
        assert_eq!(command.name, "todo");
        let mut context: Context = Context::new();
        (command.handler)(
            &mut context,
            vec![
                "create".to_string(),
                "test1".to_string(),
                "checklist1".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(context.todos.len(), 1);
        assert_eq!(context.todos.get(&1).unwrap().title, "test1");
        assert_eq!(context.todos.get(&1).unwrap().checklist.len(), 1);
    }

    //test delete
    #[test]
    fn test_todo_delete() {
        let command = CommandBuilder::todo();
        assert_eq!(command.name, "todo");
        let mut context: Context = Context::new();
        (command.handler)(
            &mut context,
            vec![
                "create".to_string(),
                "test1".to_string(),
                "checklist1".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(context.todos.len(), 1);
        assert_eq!(context.todos.get(&1).unwrap().title, "test1");
        assert_eq!(context.todos.get(&1).unwrap().checklist.len(), 1);

        (command.handler)(&mut context, vec!["delete".to_string(), "1".to_string()]).unwrap();
        assert_eq!(context.todos.len(), 0);
    }

    //test update
    #[test]
    fn test_todo_update() {
        let command = CommandBuilder::todo();
        assert_eq!(command.name, "todo");
        let mut context: Context = Context::new();
        (command.handler)(
            &mut context,
            vec![
                "create".to_string(),
                "test1".to_string(),
                "checklist1".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(context.todos.len(), 1);
        assert_eq!(context.todos.get(&1).unwrap().title, "test1");
        assert_eq!(context.todos.get(&1).unwrap().checklist.len(), 1);
        (command.handler)(
            &mut context,
            vec![
                "update".to_string(),
                "status".to_string(),
                "1".to_string(),
                "true".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(context.todos.len(), 1);
        assert_eq!(context.todos.get(&1).unwrap().title, "test1");
        assert_eq!(context.todos.get(&1).unwrap().checklist.len(), 1);
        assert_eq!(context.todos.get(&1).unwrap().completed, true);
    }

    //test todo update checklist
    #[test]
    fn test_todo_update_checklist() {
        let command = CommandBuilder::todo();
        assert_eq!(command.name, "todo");
        let mut context: Context = Context::new();
        (command.handler)(
            &mut context,
            vec![
                "create".to_string(),
                "test1".to_string(),
                "checklist1".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(context.todos.len(), 1);
        assert_eq!(context.todos.get(&1).unwrap().title, "test1");
        assert_eq!(context.todos.get(&1).unwrap().checklist.len(), 1);
        (command.handler)(
            &mut context,
            vec![
                "update".to_string(),
                "checklist".to_string(),
                "1".to_string(),
                "checklist1".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(context.todos.len(), 1);
        assert_eq!(context.todos.get(&1).unwrap().title, "test1");
        assert_eq!(context.todos.get(&1).unwrap().checklist.len(), 1);
        assert_eq!(
            context
                .todos
                .get(&1)
                .unwrap()
                .checklist
                .get("checklist1")
                .unwrap(),
            &true
        );
    }
}
