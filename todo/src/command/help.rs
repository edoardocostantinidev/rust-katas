use crate::command::get_all_command_descriptions;
use crate::command::*;

pub struct Help {}
impl Command for Help {
    fn new() -> Help {
        Help {}
    }

    fn run(&self, _args: &[&str]) {
        for elem in get_all_command_descriptions() {
            println!("{}", elem);
        }
    }
}

impl Describe for Help {
    fn describe(&self) -> String {
        String::from("help - Prints all available commands")
    }
}
