use std::{io::Write, net::TcpStream};
use std::process;
pub struct Client{
    connection : TcpStream
}


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
            r#"List CREDENTIAL"#,
        );
        let size = self.connection.write(proto.as_bytes()).unwrap_or_else(|_buf|{
            0
        });
        if size == 0{
            println!("ERROR: Unable to fetch credential from the server")
        }
    }

}