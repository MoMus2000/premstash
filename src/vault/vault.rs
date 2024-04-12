use std::fs;
use std::io::{Read, Write};
use std::process;
use std::io::{Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use std::io;

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
            Ok(_created) => {
                println!("Directory created");
                let file_path = format!("{}", FILE_PATH);
                file = match OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(&file_path) {
                        Ok(file) => file,
                        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
                            println!("File already exists");
                            file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .read(true)
                            .open(FILE_PATH)
                            .unwrap();
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

    pub fn write_to_vault(&mut self, credential: String) {
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

    pub fn read_from_vault(&mut self, credential: String) -> Option<String>{
        let mut buffer = String::new();
        match self.file.read_to_string(&mut buffer) {
            Ok(_content) => {
                let credential = Vault::search_by_key(buffer, credential);
                self.file.seek(SeekFrom::Start(0)).unwrap_or_else(|_|{
                    println!("Could not flush the file");
                    0
                });
                return credential
            },
            Err(_) => {
                println!("File is closed.");
                // Handle the file being closed
            }
        } 
        None
    }

    pub fn list_keys_from_vault(&mut self) -> Option<Vec<String>>{
        let mut buffer = String::new();
        match self.file.read_to_string(&mut buffer) {
            Ok(_content) => {
                println!("File is still open.");
                let filtered_keys = Vault::filter_keys(buffer);
                self.file.seek(SeekFrom::Start(0)).unwrap_or_else(|_|{
                    println!("Could not flush the file");
                    0
                });
                return Some(filtered_keys)
            },
            Err(err) => {
                println!("File is closed. {}", err);
            }
        } 
        None
    }

    pub fn filter_keys(file_string: String) -> Vec<String>{
        let mut key_store = Vec::<String>::new();
        println!("file_string {:?}", file_string);
        for line in file_string.lines(){
            let key : Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
            println!("filter keys {:?}", key);
            match key.get(1){
                Some(exists) => key_store.push(exists.to_string()),
                None => println!("ERROR: Nothing to list")
            }
        }
        key_store
    }

    pub fn search_by_key(file_string: String, credential_key: String) -> Option<String>{
        for line in file_string.lines(){
            let key : Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
            match key.get(1){
                Some(exists) => {
                    println!("Candidate {}", exists);
                    println!("CRED_KEY {}", credential_key);
                    if exists.to_string() == credential_key{
                        println!("CREDENTIAL FOUND: {}", credential_key);
                        let value = key.get(3).unwrap();
                        return Some(value.to_string())
                    }
                }
                None => println!("ERROR: Nothing to list")
            }
        }
        None
    }

}

#[cfg(test)]
mod tests{
    use crate::vault::encrypt;

    #[test]
    fn test_encrypt_decrypt(){
        let mut enc = encrypt::Enc::new("magickey");
        let enc_string = enc.encrypt("Something Something 69".to_string());
        let dec = enc.decrypt(enc_string.clone());
        println!("Encrypted String : {:?}", enc_string);
        println!("Decrypted String : {}", dec);
    }

}
