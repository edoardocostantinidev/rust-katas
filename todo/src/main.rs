use todo::handler;
use todo::repository::TodoRepository;
use todo::service::TodoService;
fn main() {
    let todo_repo = TodoRepository::new();
    let todo_service = TodoService::new(todo_repo);
    handler::run(todo_service);
}
