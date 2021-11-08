pub mod exit;
pub mod help;
pub mod todo;

pub trait Command {
    fn run(&mut self, args: &[&str]);
}
pub trait Describe {
    fn describe() -> String;
}

pub fn get_all_command_descriptions() -> Vec<String> {
    vec![
        help::Help::describe(),
        exit::Exit::describe(),
        todo::Todo::describe(),
    ]
}
