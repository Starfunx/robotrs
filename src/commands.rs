

use hashbrown::HashMap;
use alloc::vec::Vec;


pub struct Command {
    name: &'static str,
    handler: fn(&[&str]) -> &'static str,
}


impl Command {
    pub fn new(name: &'static str, handler: fn(&[&str]) -> &'static str) -> Command {
        Command {
            name,
            handler,
        }
    }
}


pub struct CommandParser {
    commands: HashMap<&'static str, Command>,
}

impl CommandParser {
    pub fn new(commands: HashMap<&'static str, Command>) -> CommandParser {
        CommandParser { commands }
    }

    pub fn parse(&self, command_string: &str) -> &'static str {
        let mut tokens = command_string.split_whitespace();
        let command_name = tokens.next().unwrap_or("");
        let command = self.commands.get(command_name);

        match command {
            Some(c) => (c.handler)(tokens.collect::<Vec<_>>().as_slice()),
            None => ""
        }

    }
}
