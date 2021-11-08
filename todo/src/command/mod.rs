pub mod exit;
pub mod help;

pub trait Command {
    fn new() -> Self;
    fn run(&self, args: &[&str]);
}
pub trait Describe {
    fn describe(&self) -> String;
}

pub fn get_all_command_descriptions() -> Vec<String> {
    let help = help::Help::new();
    let exit = exit::Exit::new();
    vec![help.describe(), exit.describe()]
}
