use std::{borrow::BorrowMut, io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::thread;
use std::str;

use crate::server::parser::Parser;
use crate::vault::vault::Vault;

pub struct Server{
    listener: TcpListener,
    vault: Vault
}

impl Server{
    pub fn new(port: String) -> Self{
        let vault = Vault::new();
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port));
        let listener = listener.expect("ERROR: Could not initialize server on the desired port");
        Server {listener: listener, vault}
    }

    pub fn serve(&self){
        for stream in self.listener.incoming(){
            match stream{
                Ok(tcp_stream_acquired) => {
                    println!("Connection established with {:?}", tcp_stream_acquired.local_addr());
                    let mut vault = self.vault.clone();
                    thread::spawn(move ||{
                        handle_connection(tcp_stream_acquired, vault.borrow_mut());
                    });
                }

                Err(_) => {
                    println!("ERROR: Could not process incoming client request")
                }

            }
        }
    }
}

fn handle_connection(mut conn_stream: TcpStream, vault: &mut Vault){
    let mut buffer = [0; 512];
    loop {
        let vault = vault.borrow_mut();
        let len = match conn_stream.read(&mut buffer) {
            Ok(len) if len == 0 => {
                println!("Connection closed by peer");
                break; // Break out of the loop if the stream has ended
            }
            Ok(len) => len,
            Err(err) => {
                eprintln!("Error reading from stream: {}", err);
                break; // Break out of the loop on error
            }
        };
        let string_result = str::from_utf8(&buffer[0..len]).expect("Failed to convert to UTF-8");
        let parsed_command = Parser::parse(string_result.to_string());

        match parsed_command{
            Ok(res) => {
                println!("Message from server: {}", string_result);
                println!("Running the associated fun");
                let output = (res.command.associate_func)(vault, string_result.to_string());
                match output{
                    Some(res) => {
                        match conn_stream.write_all(res.as_bytes()){
                            Ok(_) => {
                                println!("Sending List to the client")
                            }
                            Err(err) => {
                                println!("ERROR: {}", err)
                            }
                        }
                        println!("Output: {}", res)
                    }
                    None => {
                        conn_stream.write_all("ERROR: Did not find your credential ..".as_bytes()).unwrap();
                        break
                        println!("ERROR: got nothing back")
                    }
                }
            }
            Err(_) => {
                break
            }
        }
    }
}