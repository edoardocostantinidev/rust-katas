use crate::command::*;

pub struct Exit {}
impl Command for Exit {
    fn run(&mut self, _args: &[&str]) {
        println!("bye bye! 👋");
        std::process::exit(0);
    }
}
impl Describe for Exit {
    fn describe() -> String {
        return String::from("exit - exit the program");
    }
}
