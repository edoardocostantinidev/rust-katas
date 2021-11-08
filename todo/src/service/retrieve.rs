use crate::domain::todo::Todo;
use crate::service::TodoService;
impl TodoService {
    pub fn retrieve(&self, id: i32) -> Option<Todo> {
        self.todo_repo.retrieve(id)
    }
}
