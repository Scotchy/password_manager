
use std::fs::File;
use std::path::Path;

use std::io::{self, Read, Write};
use rand::Rng;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum EncryptedType {
    Account { id : u32, name : String, password : String }, //Implement interior mutability with RefCell.
    Note { id : u32, name : String, content : String }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct EncryptedWallet {
    name : String,
    pub encrypted_content : String
}

impl EncryptedWallet {

    pub fn to_json(&self) -> Result<String, serde_json::error::Error> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }
    pub fn from_json(json : &str) -> Result<EncryptedWallet, serde_json::error::Error> {
        let wallet : EncryptedWallet = serde_json::from_str(json)?;
        Ok(wallet)
    }
    pub fn decrypt(&self, password : &str, iv : &[u8]) -> Result<Wallet, i32> {
        let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            password.as_bytes(),
            iv,
            blockmodes::PkcsPadding);  
        
        let encrypted_wallet = base64::decode(&self.encrypted_content).unwrap();

        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(encrypted_wallet.as_slice());
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }
        let json = String::from_utf8(final_result).unwrap();
        let wallet = Wallet::from_json(&json[..]).unwrap();
        Ok(wallet)
    }

    pub fn save(&self, path : &str) -> Result<(), i32> {
        let mut path = Path::new(path);
        
        if !path.is_file() {
            return Err(1);
        }
        let mut file = File::create(path).unwrap();
        file.write_all(self.to_json().unwrap().as_bytes());
        Ok(())
    }
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub name : String,
    pub content : Vec<EncryptedType>   
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Wallet {
    
    pub fn check_name(name : &str) -> bool {
        true
    }

    pub fn create(folder : &str, name  : &str, password : &str) -> Result<(), i32> {
        
        if !Wallet::check_name(name) {
            return Err(0);
        }
        let mut path = Path::new(folder);
        
        if !path.is_dir() {
            return Err(1);
        }

        let mut file = name.to_string();
        file.push_str(".spw");

        let mut path = path.to_str().unwrap().to_string();
        path.push_str("/");
        path.push_str(&file[..]); 
        let path = Path::new(&path);

        let mut file = File::create(path).unwrap();
    
        let mut json_init = "Wallet { name: \"".to_string();
        json_init.push_str(name);
        json_init.push_str("\", content: [] }");

        let wallet = Wallet { name: name.to_string(), content : vec!() };
        
        file.write_all(wallet.encrypt(password, "iviviviviviviviv".as_bytes()).unwrap().to_json().unwrap().as_bytes()).unwrap();
        // Vecteur IV
        Ok(())
    }

    pub fn encrypt(&self, password : &str, iv : &[u8]) -> Result<EncryptedWallet, io::Error> {
        let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256, 
            password.as_bytes(), 
            iv, 
            blockmodes::PkcsPadding);
        
        let json_wallet = self.to_json().unwrap();
        let json_wallet = json_wallet.as_bytes();

        let mut encrypted_content = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(json_wallet);
        let mut buffer = [0; 4096];
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
        
        loop {
            let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
            encrypted_content.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }
        let encrypted_content = base64::encode(&encrypted_content);
        Ok(EncryptedWallet { name : self.name.clone(), encrypted_content })
    }

    pub fn open(path : &str, password : &str) -> Result<Wallet, i32> {

        let mut f = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(0)
        };
        let mut v = String::new();
        f.read_to_string(&mut v).unwrap();
        let encrypted_wallet = EncryptedWallet::from_json(&v[..]).unwrap();
        
        let wallet = encrypted_wallet.decrypt(password, "iviviviviviviviv".as_bytes()).unwrap();
        Ok( wallet )
    }
    
    pub fn add_note(&mut self, id : u32, name : String, content : String) {
        self.content.push(EncryptedType::Note { id, name, content });
    }

    pub fn empty() -> Wallet {
        Wallet { name : String::new(), content : Vec::new() }
    }

    fn to_json(&self) -> Result<String, serde_json::error::Error> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }

    pub fn from_json(json : &str) -> Result<Wallet, serde_json::error::Error> {
        let wallet : Wallet = serde_json::from_str(json)?;
        Ok(wallet)
    }

    pub fn get_by_id(&self, sid : u32) -> Option<&EncryptedType> {
        for el in &self.content {
            match el {
                EncryptedType::Account { id, .. } if *id == sid => Some(el),
                EncryptedType::Note { id, .. } if *id == sid=> Some(el),
                _ => continue
            };
        }
        None
    }

    pub fn save(&self, path : &str, password : &str, iv : &[u8]) {
        let encrypted = self.encrypt(password, iv).unwrap(); 
        encrypted.save(path);
    }

    pub fn delete_permanently(password : &String) -> Result<(), i32> {
        Ok(())
    }
    
}

pub struct WalletManager {
    pub wallets : Vec<Wallet>
}

impl WalletManager {
    pub fn new() -> WalletManager {
        WalletManager { wallets : Vec::new() }
    }

    pub fn open_wallet(&mut self, path : &str, password : &str) {
        let wallet = Wallet::open(path, password).unwrap();
        self.wallets.push(wallet);
    }

    pub fn add_note(&mut self, wid : usize, id : u32, name : String, content : String) {
        &self.wallets[wid].add_note(id, name, content);
    }

    pub fn show(&self, wid : usize) {
        print!("{:?}\n", self.wallets[wid]);
    }
}