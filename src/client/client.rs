use std::{io::Write, net::TcpStream};
use std::thread;
use std::time;
pub struct Client{
    connection : TcpStream
}


impl Client{
    pub fn new(port: String) -> Self{
        let connection = TcpStream::connect(format!("127.0.0.1:{}", port));
        let connection = connection.expect("ERROR: unable to establish connection with the server");
        Client{connection}
    }

    pub fn listen_and_poll(&mut self, msg: String){
        loop{
            let buffer = msg.as_bytes();
            self.connection.write_all(buffer).unwrap_or_else(|_buf| println!("Error: could not write to the server"));
            thread::sleep(time::Duration::from_secs(5));
        }
    }
}