use crate::command::*;
use crate::service::*;

pub struct Todo {
    todo_service: TodoService,
}
impl Todo {
    pub fn new(todo_service: TodoService) -> Self {
        Self {
            todo_service: todo_service,
        }
    }
}

impl Command for Todo {
    fn run(&mut self, args: &[&str]) {
        match args[0] {
            "create" => {
                let title = &args[1];
                let description = &args[2..].join(" ");
                self.todo_service
                    .create(title.to_string(), description.to_string());
            }
            "retrieve" => {
                let id = (&args[1]).parse::<i32>().unwrap().clone();
                let todo = self.todo_service.retrieve(id).unwrap();
                println!("{}", todo);
            }
            "delete" => self.todo_service.delete(),
            "update" => self.todo_service.update(),
            "help" => println!("available Todo commands:\ncreate\nretrieve\ndelete\nupdate\nhelp"),
            _ => eprintln!("Invalid Todo command"),
        }
    }
}
impl Describe for Todo {
    fn describe() -> String {
        String::from("todo: CRUD for todos. Use todo help to receive further help on todo command")
    }
}
