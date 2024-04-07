use clap::*;

mod client;
mod server;
fn main() {

    let default_port = &"8080".to_string();
    let default_ip_address = &"127.0.0.1".to_string();
    let matches = command!() // requires `cargo` feature
        .subcommand_required(true)
        .subcommand(
            serve_command()
        )
        .subcommand(
            client_command()
        )
        .get_matches();

    
    match matches.subcommand(){
        Some(("serve", sub_matches)) =>{
            let port = sub_matches.get_one::<String>("port");
            let port = port.unwrap_or_else(|| {
                default_port
            });
            let server = server::serve::Server::new(port.clone());
            server.serve();
        }
        Some(("connect", sub_matches)) =>{
            let port = sub_matches.get_one::<String>("port");
            let port = port.unwrap_or_else(|| {
                default_port
            });
            let ip_address = sub_matches.get_one::<String>("target-ip");
            let ip_address = ip_address.unwrap_or_else(|| {
               default_ip_address 
            });
            let mut client = client::client::Client::new(ip_address.to_string(), port.clone());

            match sub_matches.subcommand(){
                Some(("get", command)) =>{
                    let argument = command.get_one::<String>("credential").unwrap();
                    client.get_cred(argument.to_string());
                }
                Some(("push", command)) =>{
                    let argument = command.get_one::<String>("credential").unwrap();
                    client.push_cred(argument.to_string());
                }
                Some(("list", command)) =>{
                    println!("List command: {:?}", command);
                    client.list_credential();
                }
                _ => {
                    println!("ERROR: command not found")
                }
            }
        }
        _ => {

        }
    }

}

fn serve_command() -> clap::Command{
    let serve = 
        clap::Command::new("serve")
        .short_flag('s')
        .about("Serve premstash");

    let define_port = 
        clap::Arg::new("port")
        .short('p')
        .help("Define port");

    serve
    .arg(
        define_port
    )
}

fn client_command() -> clap::Command{
    let connect= 
        clap::Command::new("connect")
        .short_flag('c')
        .about("Connect to premstash");

    let get= 
        clap::Command::new("get")
        .short_flag('g')
        .about("Fetch value");

    let push= 
        clap::Command::new("push")
        .short_flag('u')
        .about("push value");

    let get_credential =
        clap::Arg::new("credential")
        .index(1)
        .help("credential to send to the server");

    let push_credential =
        clap::Arg::new("credential")
        .index(1)
        .help("credential to send to the server");

    let list= 
        clap::Command::new("list")
        .short_flag('l')
        .about("list all stored credentials");

    let define_port = 
        clap::Arg::new("port")
        .short('p')
        .help("Define port");

    let target_ip = 
        clap::Arg::new("target-ip")
        .short('i')
        .help("Define target ip");

    connect 
    .arg(
        define_port
    )
    .arg(
        target_ip
        .required(true)
    )
    .subcommand(
        get
        .arg(
            get_credential
            .required(true)
        )
    )
    .subcommand(
        push
        .arg(
            push_credential
            .required(true)
        )
    )
    .subcommand(
       list 
    )
}