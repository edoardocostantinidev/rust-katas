use crate::model::Todo;
use std::collections::HashMap;
#[derive(Clone)]
pub struct Context {
    pub todos: HashMap<i32, Todo>,
    pub last_id: i32,
}
impl Context {
    pub fn new() -> Self {
        Self {
            todos: HashMap::new(),
            last_id: 0,
        }
    }
}
