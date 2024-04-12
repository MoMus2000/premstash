use short_crypt::ShortCrypt;

pub struct Enc{
    pub MAGIC: ShortCrypt,
    pub BASE: Option<u8>
}

impl Enc{
    pub fn new(key: &str) -> Self{
        let key = ShortCrypt::new(key);
        Enc{MAGIC: key, BASE: None}
    }

    pub fn encrypt(&mut self, encryption_string: String) -> Vec<u8> {
        let encrypted_string = self.MAGIC.encrypt(encryption_string.as_str());
        let u8 = encrypted_string.0;
        self.BASE = Some(u8);
        let encrypted_string = encrypted_string.1;
        encrypted_string
    }

    pub fn decrypt(&self, encryption_string: Vec<u8>) -> String {
        let decrypt_value : Vec<u8> = encryption_string;
        let decrypt_string = (self.BASE.unwrap(), decrypt_value);
        let decrypted_string = self.MAGIC.decrypt(&decrypt_string);
        let decrypted_string = decrypted_string.expect("Could not decrypt");
        let decrypted_string : String = String::from_utf8(decrypted_string).unwrap();
        decrypted_string
    }

}
