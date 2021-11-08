// struct todo with description and status
struct Todo {
    description: String,
    status: TodoStatus,
    create_date: DateTime<Utc>,
    update_date: DateTime<Utc>,
    // checklist dictionary
    checklist: HashMap<u32, ChecklistItem>,
}

// enum todo_status with three variants
enum TodoStatus {
    Doing,
    Done,
    ToDo,
}
