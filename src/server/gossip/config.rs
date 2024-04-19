use yaml_rust::YamlLoader;
use std::fs;
use std::env;

pub struct Config {
    pub main_servers: Vec<String>,
    pub ping_interval: String,
}

pub fn read_config() -> Config{
    println!("Current dir {:?}", env::current_dir());
    let file = fs::read_to_string("./src/server/gossip/config.yml");
    let yaml = YamlLoader::load_from_str(file.unwrap().as_str()).unwrap();

    let config = &yaml[0];
    
    let mut main_servers = Vec::<String>::new();
    for server in config["main_servers"].clone().into_iter(){
        main_servers.push(server.as_str().unwrap().to_string());
    }

    let ping_interval = config["ping_interval"].as_str().unwrap().to_string();

    Config{
        main_servers,
        ping_interval
    }

}
