pub mod app;
pub mod command;
pub mod context;
pub mod model;
pub mod parser;
use app::App;

fn main() {
    println!("Starting Todo App 🚀");
    let mut app = App::new();
    app.run().expect_err("Something went wrong");
}
