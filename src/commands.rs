

use hashbrown::HashMap;


struct Command {
    name: &'static str,
    description: &'static str,
    handler: fn(&[&str]) -> &'static str,
}


impl Command {
    fn new(name: &'static str, description: &'static str, handler: fn(&[&str]) -> &'static str) -> Command {
        Command {
            name,
            description,
            handler,
        }
    }
}


struct CommandParser {
    commands: HashMap<&'static str, Command>,
}

impl CommandParser {
    fn new() -> CommandParser {
        let mut commands = HashMap::new();
        commands.insert("help", Command::new("help", "Print this help message", help));
        commands.insert("set_led", Command::new("set_led", "Set the state of the LED", set_led));
        CommandParser { commands }
    }

    fn parse(&self, command_string: &str) -> &'static str {
        let mut tokens = command_string.split_whitespace();
        let command_name = tokens.next().unwrap_or("");
        let command = self.commands.get(command_name);

        match command {
            Some(c) => (c.handler)(tokens.collect::<Vec<_>>().as_slice()),
            None => "Error: Invalid command"
        }
    }
}


fn help(_: &[&str]) -> &'static str {
    "Available commands:\nhelp: Print this help message\nset_led: Set the state of the LED\n"
}

fn set_led(args: &[&str]) -> &'static str {
    if args.len() != 1 {
        return "Error: set_led command takes exactly 1 argument";
    }
    match args[0] {
        "on" => {
            // set the LED on
            "LED turned on"
        }
        "off" => {
            // set the LED off
            "LED turned off"
        }
        _ => "Error: Invalid argument. Use 'on' or 'off'",
    }
}
