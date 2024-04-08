use crate::vault::vault::Vault;

pub struct Command{
    pub associate_func: fn(&mut Vault, String) -> Option<String>,
}

pub fn get_credential(vault: &mut Vault, credential: String) -> Option<String>{
    println!("Fetching credentials {}", credential);
    None
}

pub fn push_credential(vault: &mut Vault, credential: String) -> Option<String>{
    println!("Pushing credentials {}", credential);
    vault.write_to_vault("something".to_string(), "ok".to_string());
    None
}

pub fn list_credential(vault: &mut Vault, _credential: String) -> Option<String>{
    None
}

impl Command{
    pub fn route_command(method: &str) -> Option<Command>{
        match method{
            "GET" => {
                return Some(
                    Command{
                        associate_func: get_credential,
                    }
                )
            }
            "PUSH" => {
                return Some(
                    Command{
                        associate_func: push_credential,
                    }
                )
            }
            "LIST" => {
                return Some(
                    Command{
                        associate_func: list_credential,
                    }
                )
            }
            _ => {
                return None
            }
        }
    }
}