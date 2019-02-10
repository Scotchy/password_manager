#[macro_use]
extern crate serde_derive;
extern crate serde_json; 
extern crate crypto;
extern crate nat_messaging;
extern crate rand;
extern crate base64;
#[macro_use]
extern crate clap;

mod wallet_manager;
mod lib; 

use std::io::{self, Write, Read};
use std::fs::File;
use std::process;
use std::env;
use clap::App;
use wallet_manager::Wallet; 
use std::error::Error;
use crate::wallet_manager::EncryptedType;

fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("args.yml"); 
    let matches = App::from(yaml).get_matches();

    let current_dir = env::current_exe()?;
    let current_dir = current_dir.parent().or(Some(&current_dir)).unwrap();
    let current_dir = current_dir.to_str().or(Some(&"")).unwrap().to_string();

    let mut wm = wallet_manager::WalletManager::new();

    if let Some(wallet_file) = matches.value_of("open_from_path") {
        if let Some(password) = matches.value_of("password") {
            let mut wallet = Wallet::open(wallet_file, password).unwrap();

            if matches.is_present("read_passwords") {
                read_passwords(&wallet); 
            }
        }
    }
    if let Some(wallet_name) = matches.value_of("open") {
        if let Some(password) = matches.value_of("password") {
            let mut wallet_file = current_dir.clone();
            wallet_file.push_str("/wallets/"); 
            wallet_file.push_str(wallet_name);
            wallet_file.push_str(".spw"); 
            print!("Opening wallet at {}\n", wallet_file);
            let mut wallet = Wallet::open(&wallet_file[..], password).unwrap();

            let mut must_save = false;

            if matches.is_present("read_passwords") {
                read_passwords(&wallet); 
            }
            if let Some(new_password) = matches.value_of("add_password") {
                if let Some(label) = matches.value_of("label") {
                    add_password(&mut wallet, label, new_password); 
                    must_save = true;
                }
            }

            if must_save {
                wallet.save(&wallet_file[..], password, "iviviviviviviviv".as_bytes());
            }
        }
    }

    if let Some(wallet_name) = matches.value_of("create") {
        if let Some(password) = matches.value_of("password") {
            if let Some(wallet_folder) = matches.value_of("folder") {
                print!("Creating new wallet \"{:}\" in directory \"{:}\"\n", wallet_name, wallet_folder);
                Wallet::create(wallet_folder, wallet_name, password).unwrap(); 
            }       
            else {
                print!("Creating new wallet \"{:}\" in default directory \"./wallets\"\n", wallet_name);
                Wallet::create("wallets", wallet_name, password).unwrap();
            }
        }
    }

    /*
    let mut pass = String::new();
    print!("\n"); 
    std::io::stdin().read_line(&mut pass)?;

    wm.open_wallet("test2.txt".to_string(), &pass);
    //wm.add_note(0, 4, "Hey !".to_string(), "Salut !".to_string());
    //wm.show(0);
    let ew = wm.wallets[0].encrypt("password", "ahah".as_bytes())?; 
    print!("{:?}", ew);*/
    Ok(())
    /*let mut i = 0;
    loop {
        let message = match nat_messaging::read_stdin() {
            Ok(m) => m,
            Err(_) => process::exit(1),
        };
        //println_stderr!("received {}", message);
        let answer = format!("\"{} {}\"", i, "Bonjour");
        nat_messaging::write_stdout(&answer);
        i += 1;
    }*/
}

pub fn read_passwords(wallet : &Wallet) {
    print!("Passwords: \n");
    for c in wallet.content.iter() {
        match c {
            EncryptedType::Account { id: _, name, password } => print!("{}: {}\n", name, password),
            _ => ()
        }; 
    }
    
}

pub fn add_password(wallet : &mut Wallet, name : &str, password : &str) {
    wallet.content.push(EncryptedType::Account { id: 0, name: name.to_string(), password: password.to_string() });
}