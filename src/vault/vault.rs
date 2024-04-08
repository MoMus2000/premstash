use std::fmt::format;
use std::fs;
use std::io::{Read, Write};
use std::process;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub struct Vault{
    pub file: File
}

const CREDENTIAL_DIR : &str = "./premstash";
const FILE_PATH: &str = "./premstash/cred.vault";

impl Clone for Vault {
    fn clone(&self) -> Self {
        let cloned_file = match self.file.try_clone() {
            Ok(file) => file,
            Err(_) => panic!("Failed to clone file."),
        };
        Vault {
            file: cloned_file,
        }
    }
}

impl Vault{
    pub fn new() -> Vault{
        let mut file;
        match fs::create_dir_all(CREDENTIAL_DIR){
            Ok(created) => {
                println!("Directory created");
                let file_path = format!("{}", FILE_PATH);
                file = match OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(&file_path) {
                        Ok(file) => file,
                        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
                            println!("File already exists");
                            file = File::open(FILE_PATH).expect("File should open");
                            file
                        },
                        Err(err) => {
                            eprintln!("ERROR: Could not create/open vault file: {}", err);
                            process::exit(1);
                        }
                };
            }
            Err(err) => {
                println!("ERROR: {}", err);
                process::exit(1);
            }
        }
        Vault{
            file
        }
    }

    pub fn write_to_vault(&mut self, key: String, value: String) {
        let credential = format!("KEY:{} VALUE:{}", key, value);
        match self.file.write(credential.as_bytes()) {
            Ok(size) => {
                if size == 0 {
                    println!("ERROR: unable to write credential");
                }
            }
            Err(err) => {
                println!("ERROR: Failed to write credential: {}", err);
            }
        }
    }

    pub fn read_from_vault(&self, credential: String){
        self.file.read_to_end(buf)
    }

}
