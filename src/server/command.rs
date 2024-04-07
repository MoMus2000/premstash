pub struct Command{
    command: String,
    associate_func: fn(String) -> Option<String>,
}

fn get_credential(credential: String) -> Option<String>{
    None
}

fn push_credential(credential: String) -> Option<String>{
    None
}

fn list_credential() -> Option<String>{
    None
}

impl Command{
    fn route_command(&self, command_string: &str) -> Option<Command>{
        match command_string{
            "get_credential" => {
                return Some(
                    Command{
                        command: command_string.to_string(),
                        associate_func: get_credential,
                    }
                )
            }
            _ => {
                return None
            }
        }
    }
}