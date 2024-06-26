use crate::vault::vault::Vault;

pub struct Command{
    pub associate_func: fn(String) -> Option<String>,
}

pub fn delete_credential(credential: String) -> Option<String>{
    let mut vault = Vault::new();
    let credential : Vec<&str> = credential.split(":").into_iter().collect();
    let credential = credential.get(1).unwrap();
    println!("DELETING credentials {}", credential);
    let fetched_credential = vault.delete_keys_from_vault(credential.to_string());
    println!("Deleted : {:?}", fetched_credential);
    match fetched_credential{
        Some(res) => return Some(res.to_string()),
        _  => return Some("Could not find your credential ..".to_string())
    }
}

pub fn update_credential(credential: String) -> Option<String>{
    let mut vault = Vault::new();
    let credential : Vec<&str> = credential.split(":").into_iter().collect();
    let key = credential.get(1).unwrap();
    let value = credential.get(2).unwrap();
    let credential = vec![key, value];
    println!("UPADTE credentials {:?}", credential);
    vault.update_keys_from_vault(credential);
    None
}

pub fn get_credential(credential: String) -> Option<String>{
    let mut vault = Vault::new();
    let credential : Vec<&str> = credential.split(":").into_iter().collect();
    let credential = credential.get(1).unwrap();
    println!("Fetching credentials {}", credential);
    let fetched_credential = vault.read_from_vault(credential.to_string());
    println!("FETCHED: {:?}", fetched_credential);
    match fetched_credential{
        Some(res) => return Some(res),
        _  => return Some("Could not find your credential ..".to_string())
    }
}

pub fn push_credential(credential: String) -> Option<String>{
    let mut vault = Vault::new();
    let credential : Vec<&str> = credential.split(":").into_iter().collect();
    let key = credential.get(1).unwrap();
    let value = credential.get(2).unwrap();
    let credential = vec![key, value];
    println!("Pushing credentials {:?}", credential);
    vault.write_to_vault(credential);
    None
}

pub fn list_credential(_credential: String) -> Option<String>{
    println!("Listing credentials");
    let mut vault = Vault::new();
    let keys = vault.list_keys_from_vault();
    println!("GOT THESE KEYS: {:?}", keys);
    let mut res_string = Vec::<String>::new();
    match keys{
        Some(res) => {
            for key in res{
                let current_key = format!("{}", key);
                res_string.push(current_key);
                println!("KEY {}", key);
            }
            let result = res_string.join("\n");
            return Some(result)
        }
        None => {
            return Some("No keys found".to_string());
        }
    }
}

impl Command{
    pub fn route_command(method: &str) -> Option<Command>{
        match method{
            "GET" => {
                Some(
                    Command{
                        associate_func: get_credential,
                    }
                )
            }
            "PUSH" => {
                Some(
                    Command{
                        associate_func: push_credential,
                    }
                )
            }
            "LIST" => {
                Some(
                    Command{
                        associate_func: list_credential,
                    }
                )
            }
            "DELETE" => {
                Some(
                    Command{
                        associate_func: delete_credential,
                    }
                )
            }
            "UPDATE" => {
                Some(
                    Command{
                        associate_func: update_credential,
                    }
                )
            }
            _ => {
                None
            }
        }
    }
}