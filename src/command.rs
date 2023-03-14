use std::env;
use std::io;
use std::collections;

#[derive(Debug)]
pub struct Command {
    pub action: String,
    pub arguments: collections::HashMap<String, String>
}

impl Command {
    pub fn parse() -> Result<Command, io::Error> {
        let mut command: Command = Command { action: String::new(), arguments: collections::HashMap::new() };
        let mut last_argument: String = String::new();
        for argument in env::args().skip(1) {
            //if theres no action, take the first argument as action
            if command.action == String::new() {
                command.action = argument;
            } else {
                //if there is a new argument -- but last_argument was not set, place a default value
                if last_argument != String::new() && argument.starts_with("--") {
                    command.arguments.insert(last_argument, String::from("true"));
                    last_argument = argument.replace("--", "");
                    command.arguments.insert(last_argument.clone(), String::new());
                } else {
                    //parse new argument
                    if argument.starts_with("--") {
                        last_argument = argument.replace("--", "");
                        command.arguments.insert(last_argument.clone(), String::new());
                    } else {
                        //if last argument was set complete it
                        if last_argument != String::new() {
                            command.arguments.insert(last_argument, argument);
                            last_argument = String::new();
                        } else {
                            //you must not be here
                            let error: io::Error = io::Error::new(io::ErrorKind::Other, format!("argument {} is not parsable", argument));
                            return Err(error);
                        }
                    }
                }
            }
        }
        Ok(command)
    }
}
