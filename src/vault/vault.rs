use std::fs;
use std::io::{Read, Write};
use std::process;
use std::io::{Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use std::io;

use crate::vault::sqlite::db::sqlite_db;

#[derive(Debug)]
pub struct Vault{
    pub db: sqlite_db
}

const CREDENTIAL_DIR : &str = "./premstash";
const FILE_PATH: &str = "./premstash/cred.vault";

impl Vault{
    pub fn new() -> Vault{
        let db = sqlite_db::new();
        Vault{
            db
        }
    }

    pub fn write_to_vault(&mut self, credential: Vec<&&str>) {
        self.db.insert_cred(credential);
    }

    pub fn read_from_vault(&mut self, credential: String) -> Option<String>{
        Some(self.db.fetch_cred(&[credential]))
    }

    pub fn list_keys_from_vault(&mut self) -> Option<Vec<String>>{
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
