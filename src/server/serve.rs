use std::{fmt::format, io::Read, net::{TcpListener, TcpStream}};
use std::thread;
use std::str;
pub struct Server{
    listener: TcpListener,
}

impl Server{
    pub fn new(port: String) -> Self{
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port));
        let listener = listener.expect("ERROR: Could not initialize server on the desired port");
        Server {listener: listener}
    }

    pub fn serve(&self){
        for stream in self.listener.incoming(){
            match stream{
                Ok(tcp_stream_acquired) => {
                    println!("Connection established with {:?}", tcp_stream_acquired.local_addr());
                    thread::spawn(move ||{
                        handle_connection(tcp_stream_acquired);
                    });
                }

                Err(_) => {
                    println!("ERROR: Could not process incoming client request")
                }

            }
        }
    }
}

fn handle_connection(mut conn_stream: TcpStream){
    let mut buffer = [0; 512];
    loop{
        let len = conn_stream.read(&mut buffer).unwrap();
        let string_result = str::from_utf8(&buffer[0..len]).expect("msg");
        println!("Message from server {}", string_result);
    }
}