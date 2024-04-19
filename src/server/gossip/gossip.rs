use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::{io::Read, net::TcpListener};
use std::str::from_utf8;

pub struct GossipServer{
    pub listener : TcpListener,
    pub port: String
}

pub fn new(server_port: usize) -> GossipServer{
    let gossip_port = server_port+1;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", gossip_port));
    match listener{
        Ok(listener_ok) => {
            return GossipServer{
                listener: listener_ok,
                port: format!(":{}",gossip_port)
            }
        }
        Err(err) => {
            panic!("ERROR: {}", err);
        }
    }
}

impl GossipServer{
    pub fn start(&self){
        println!("Starting the gossip server @ {}", self.port);
        let mut buffer = [0 as u8; 512];
        for conn in self.listener.incoming(){
            match conn{
                Ok(mut resultant_conn) => {
                    let bytes_read = resultant_conn.read(&mut buffer).unwrap();
                    let conv_to_string = from_utf8(&buffer[0..bytes_read]).unwrap();
                    println!("CONV_TO_STRING: {}", conv_to_string);
                    match conv_to_string.trim(){
                        "DISCOVER" => {
                            thread::spawn(move||{
                                chatter(resultant_conn);
                            });
                        }
                        _ => {
                            println!("Could not match incoming command");
                        }
                    }
                }
                Err(err) => {
                    println!("ERROR: {}", err)
                }
            }
        }
    }
}

pub fn chatter(mut listener: TcpStream){
    let mut buffer = [0 as u8; 512];
    listener.write("I see you, lets chat\n".as_bytes()).unwrap();
    loop{
        let n_bytes = listener.read(&mut buffer).unwrap();
        println!("{}", from_utf8(&buffer[0..n_bytes]).unwrap())
    }
}
