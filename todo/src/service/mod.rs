pub mod create;
pub mod delete;
pub mod retrieve;
pub mod update;
use crate::repository::TodoRepository;
#[derive(Clone)]
pub struct TodoService {
    pub todo_repo: TodoRepository,
}

impl TodoService {
    pub fn new(todo_repo: TodoRepository) -> Self {
        TodoService { todo_repo }
    }
}
