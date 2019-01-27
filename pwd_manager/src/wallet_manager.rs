
use std::fs::File;
use std::io::{self, Read, Write};

use std::collections::HashMap;

use std::str::from_utf8;
use std::clone;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum EncodedType {
    Account { id : u32, name : String, password : String },
    Note { id : u32, name : String, content : String }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Wallet {
    content : Vec<EncodedType>
    
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Wallet {
    
    pub fn create(name  : &String, password : &String) -> Result<(), i32> {
        Ok(())
    }

    pub fn open(path : &String, password : &String) -> Result<Wallet, i32> {

        let mut f = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(0)
        };
        let mut v = String::new();
        f.read_to_string(&mut v).unwrap();
        
        Ok( Wallet::from_json(&v[..]) )
    }
    
    pub fn empty() -> Wallet {
        Wallet { content : Vec::new() }
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
    wallets : Vec<Wallet>
}

impl WalletManager {
    pub fn new() -> WalletManager {
        WalletManager { wallets : Vec::new() }
    }

    pub fn open_wallet(&mut self, path : &String, password : &String) -> Result<&Wallet, i32> {
        
        let mut wallet = Wallet::open(path, password).unwrap();
        
        self.wallets.push(wallet);

        if let Some(r) = self.wallets.last() {
            Ok( r )
        }
        else {
            Err(0)
        }
    }
}