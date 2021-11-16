pub struct Parser;
use crate::command::{Command, CommandBuilder};

impl Parser {
    pub fn new() -> Parser {
        Parser
    }
    pub fn parse(&self, string: String) -> Result<Command, String> {
        let string = string.trim();
        if string.len() == 0 {
            return Err("Empty string".to_string());
        }
        // split a string into a vector of strings
        let words: Vec<&str> = string.split_whitespace().collect();

        let main_command = words[0];
        let args = words.iter().skip(1).map(|x| x.to_string()).collect();
        let command: Result<Command, String> = match main_command {
            "todo" => {
                let mut todo_command = CommandBuilder::todo();
                if words[1..].len() > 0 {
                    todo_command.args = args;
                }
                Ok(todo_command)
            }
            "help" => Ok(CommandBuilder::help()),
            _ => Err("Unknown command".to_string()),
        };
        command
    }
}

// test parse function
#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::command::CommandBuilder;
    #[test]
    fn test_parse() {
        let s = "todo";
        let expected_command = CommandBuilder::todo();
        let actual_command = Parser::new().parse(s.to_string()).unwrap();
        assert_eq!(actual_command.name, expected_command.name);
    }
}
