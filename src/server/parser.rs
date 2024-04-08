use std::io::{Error, ErrorKind};
use super::command;

pub struct Parser{
    pub method: String,
    pub command: command::Command,
    pub credential: String
}

impl Parser {
    pub fn parse(command_string: String) -> Result<Self, Error> {
        let payload_split : Vec<&str> = command_string.split(" ").collect();

        let method = payload_split.get(0).unwrap_or_else(||{
            &""
        });

        let credential = payload_split.get(1).unwrap_or_else(||{
            &""
        });

        let command_func = command::Command::route_command(method)
            .ok_or_else(|| Error::new(ErrorKind::Other, "Route command failed"))?;

        Ok(Parser {
            method: method.to_string(),
            command: command_func,
            credential: credential.to_string()
        })
    }
}