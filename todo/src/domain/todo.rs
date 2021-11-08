use crate::domain::checklist_item::ChecklistItem;
use chrono::DateTime;
use chrono::Utc;
use std::collections::HashMap;
use std::fmt::Display;

pub struct Todo {
    title: String,
    description: String,
    status: TodoStatus,
    create_date: DateTime<Utc>,
    update_date: DateTime<Utc>,
    checklist: HashMap<u32, ChecklistItem>,
}

impl Todo {
    pub fn new(title: String, description: String) -> Todo {
        Todo {
            title,
            description,
            status: TodoStatus::ToDo,
            create_date: Utc::now(),
            update_date: Utc::now(),
            checklist: HashMap::new(),
        }
    }
}
impl Clone for Todo {
    fn clone(&self) -> Self {
        Todo {
            title: self.title.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
            create_date: self.create_date.clone(),
            update_date: self.update_date.clone(),
            checklist: self.checklist.clone(),
        }
    }
}
enum TodoStatus {
    Doing,
    Done,
    ToDo,
}

impl Clone for TodoStatus {
    fn clone(&self) -> Self {
        match self {
            TodoStatus::Doing => TodoStatus::Doing,
            TodoStatus::Done => TodoStatus::Done,
            TodoStatus::ToDo => TodoStatus::ToDo,
        }
    }
}
trait Describe {
    fn describe(&self);
}
impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Todo: {}", self.title)
    }
}
