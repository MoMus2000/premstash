use crate::vault::vault::Vault;

pub struct Command{
    pub associate_func: fn(&mut Vault, String) -> Option<String>,
}

pub fn get_credential(vault: &mut Vault, credential: String) -> Option<String>{
    let credential : Vec<&str> = credential.split(":").into_iter().collect();
    let credential = credential.get(1).unwrap();
    println!("Fetching credentials {}", credential);
    let fetched_credential = vault.read_from_vault(credential.to_string());
    match fetched_credential{
        Some(res) => return Some(res),
        None => return None
    }
}

pub fn push_credential(vault: &mut Vault, credential: String) -> Option<String>{
    let credential : Vec<&str> = credential.split(":").into_iter().collect();
    let key = credential.get(1).unwrap();
    let value = credential.get(2).unwrap();
    let credential = format!("{}:{}\n", key, value);
    println!("Pushing credentials {}", credential);
    vault.write_to_vault(credential.to_string());
    None
}

pub fn list_credential(vault: &mut Vault, _credential: String) -> Option<String>{
    println!("Listing credentials");
    let keys = vault.list_keys_from_vault();
    let mut res_string = Vec::<String>::new();
    match keys{
        Some(res) => {
            for key in res{
                let current_key = format!("KEY: {}", key);
                res_string.push(current_key);
                println!("KEY {}", key);
            }
            let result = res_string.join("\n");
            return Some(result)
        }
        None => println!("No keys found")
    }
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