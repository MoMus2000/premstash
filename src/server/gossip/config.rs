use yaml_rust::{YamlLoader};
use std::fs;
use std::env;

pub struct Config {
    pub main_server: Vec<String>,
    pub list_of_servers: Vec<String>,
    pub ping_interval: String,
}

pub fn read_config() -> Config{
    println!("Current dir {:?}", env::current_dir());
    let file = fs::read_to_string("./src/server/gossip/config.yml");
    let yaml = YamlLoader::load_from_str(file.unwrap().as_str()).unwrap();

    let config = &yaml[0];
    
    let mut main_server = Vec::<String>::new();
    for server in config["main_server"].clone().into_iter(){
        main_server.push(server.as_str().unwrap().to_string());
    }


    let mut list_of_servers = Vec::<String>::new();

    for server in config["list_of_servers"].clone(){
        list_of_servers.push(server.as_str().unwrap().to_string());
    }

    let ping_interval = config["ping_interval"].as_str().unwrap().to_string();

    Config{
        main_server,
        list_of_servers,
        ping_interval
    }

}

#[cfg(test)]
mod tests{
    use crate::server::gossip::gossip;

    #[test]
    pub fn test_read_config(){
        gossip::read_config();
    }
}