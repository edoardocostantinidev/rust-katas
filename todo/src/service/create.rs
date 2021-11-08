use crate::domain::todo::Todo;
use crate::service::TodoService;

impl TodoService {
    pub fn create(&mut self, title: String, description: String) {
        println!("Creating todo: {} - {}", title, description);
        let todo = Todo::new(title, description);
        self.todo_repo.create(todo);
    }
}
