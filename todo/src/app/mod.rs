use super::parser::Parser;
use crate::context::Context;
use std::io;
pub struct App {
    parser: Parser,
    context: Context,
}

impl App {
    pub fn new() -> Self {
        App {
            parser: Parser::new(),
            context: Context::new(),
        }
    }
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let input = self.read_input();
            let command = self.parser.parse(input);
            match command {
                Ok(cmd) => match (cmd.handler)(&mut self.context, cmd.args) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", e),
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }

    fn read_input(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input
    }
}
