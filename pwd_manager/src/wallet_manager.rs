
use std::fs::File;
use std::path::Path;

use std::io::{self, Read, Write};
use rand::Rng;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum EncodedType {
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

    pub fn to_json(&self) -> Result<String, i32> {
        let json = serde_json::to_string(&self).unwrap();
        Ok(json)
    }
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Wallet {
    name : String,
    content : Vec<EncodedType>   
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
        //let name = path.file_stem().unwrap().to_str().unwrap();

        let mut file = File::create(path).unwrap();
    
        let mut json_init = "Wallet { name: \"".to_string();
        json_init.push_str(name);
        json_init.push_str("\", content: [] }");

        let wallet = Wallet { name: name.to_string(), content : vec!() };
        
        file.write_all(wallet.encrypt(password, b"iv").unwrap().to_json().unwrap().as_bytes()).unwrap();
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
            let result = match encryptor.encrypt(&mut read_buffer, &mut write_buffer, true) {
                Ok(a) => a,
                Err(e) => return Err(io::Error::from(io::ErrorKind::BrokenPipe))
            };

            encrypted_content.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { }
            }
        }
        let encrypted_content = base64::encode(&encrypted_content);
        Ok(EncryptedWallet { name : self.name.clone(), encrypted_content })
    }

    pub fn open(path : String, password : &String) -> Result<Wallet, i32> {

        let mut f = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(0)
        };
        let mut v = String::new();
        f.read_to_string(&mut v).unwrap();

        Ok( Wallet::from_json(&v[..]) )
    }
    
    pub fn add_note(&mut self, id : u32, name : String, content : String) {
        self.content.push(EncodedType::Note { id, name, content });
    }

    pub fn empty() -> Wallet {
        Wallet { name : String::new(), content : Vec::new() }
    }

    fn to_json(&self) -> Result<String, i32> {
        let json = serde_json::to_string(&self).unwrap();
        Ok(json)
    }

    pub fn from_json(json : &str) -> Wallet {
        let wallet : Wallet = serde_json::from_str(json).unwrap();
        wallet
    }

    pub fn get_by_id(&self, sid : u32) -> Option<&EncodedType> {
        for el in &self.content {
            match el {
                EncodedType::Account { id, .. } if *id == sid => Some(el),
                EncodedType::Note { id, .. } if *id == sid=> Some(el),
                _ => continue
            };
        }
        None
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

    pub fn open_wallet(&mut self, path : String, password : &String) {
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