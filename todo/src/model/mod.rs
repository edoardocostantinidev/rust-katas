use std::collections::HashMap;

#[derive(Clone)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub checklist: HashMap<String, bool>,
}
