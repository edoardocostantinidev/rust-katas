use todo::app::App;

fn main() {
    println!("Starting Todo App 🚀");
    let mut app = App::new();
    app.run().expect_err("Something went wrong");
}
