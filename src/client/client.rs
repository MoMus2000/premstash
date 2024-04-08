use std::io::Read;
use std::{io::Write, net::TcpStream};
use std::process;
use std::str;
pub struct Client{
    connection : TcpStream
}

const BUFFER_SIZE: usize = 1024;

impl Client{
    pub fn new(ip_address:String, port: String) -> Self{
        let connection = TcpStream::connect(format!("{}:{}", ip_address, port));
        let connection = connection.unwrap_or_else(|_stream|{
            println!("ERROR: Unable to reach the server ..");
            process::exit(1);
        });
        Client{connection}
    }

    pub fn get_cred(&mut self, credential : String){
        let proto = format!(
            r#"GET CREDENTIAL:{}"#,
            credential
        );
        let size = self.connection.write(proto.as_bytes()).unwrap_or_else(|_buf|{
            0
        });
        if size == 0{
            println!("ERROR: Unable to fetch credential from the server")
        }
    }

    pub fn push_cred(&mut self, credential : String){
        let proto = format!(
            r#"PUSH CREDENTIAL:{}"#,
            credential
        );
        let size = self.connection.write(proto.as_bytes()).unwrap_or_else(|_buf|{
            0
        });
        if size == 0{
            println!("ERROR: Unable to fetch credential from the server")
        }
    }

    pub fn list_credential(&mut self){
        let proto = format!(
            r#"LIST CREDENTIAL"#,
        );
        let size = self.connection.write(proto.as_bytes()).unwrap_or_else(|_buf|{
            0
        });
        if size == 0{
            println!("ERROR: Unable to fetch credential from the server")
        }
        let mut buffer = [0; BUFFER_SIZE];
        match self.connection.read(&mut buffer){
            Ok(success) => {
                let string_result = str::from_utf8(&buffer[0..success]).unwrap_or_else(|_|{
                    "Nothing to show .."
                });
                println!("{}", string_result);
                if success == 0{
                    println!("HOUSTON, we have a problem");
                }
            }
            Err(err) => println!("ERROR: {}", err)
        }
    }

}