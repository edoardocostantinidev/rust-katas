use crate::domain::todo::Todo;
use std::collections::HashMap;
#[derive(Clone)]
pub struct TodoRepository {
    current_id: i32,
    todos: HashMap<i32, Todo>,
}
impl TodoRepository {
    pub fn new() -> TodoRepository {
        TodoRepository {
            current_id: 0,
            todos: HashMap::new(),
        }
    }
    pub fn create(&mut self, todo: Todo) {
        let id = self.current_id;
        self.current_id += 1;
        self.todos.insert(id, todo);
    }
    pub fn find_all(&self) -> HashMap<i32, Todo> {
        self.todos.clone()
    }
    pub fn delete(&mut self, id: i32) {
        self.todos.remove(&id);
    }
    pub fn update(&mut self, id: i32, todo: Todo) {
        self.todos.insert(id, todo);
    }
    pub fn retrieve(&self, id: i32) -> Option<Todo> {
        self.todos.get(&id).cloned()
    }
}
