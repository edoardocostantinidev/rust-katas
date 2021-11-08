use crate::command::*;

pub struct Exit {}
impl Command for Exit {
    fn new() -> Exit {
        Exit {}
    }
    fn run(&self, args: &[&str]) {
        println!("bye bye! 👋");
        std::process::exit(0);
    }
}
impl Describe for Exit {
    fn describe(&self) -> String {
        return String::from("exit: exit the program");
    }
}
