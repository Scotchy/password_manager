#[macro_use]
extern crate serde_derive;
extern crate serde_json; 
extern crate crypto;
extern crate nat_messaging;
extern crate rand;
extern crate base64;
#[macro_use]
extern crate clap;

use std::io::{self, Write, Read};
use std::fs::File;
use std::process;
use clap::App;

mod wallet_manager;

fn main() -> io::Result<()> {
    let yaml = load_yaml!("args.yml"); 
    let matches = App::from(yaml).get_matches();

    let mut wm = wallet_manager::WalletManager::new();

    if let Some(wallet_name) = matches.value_of("create") {
        if let Some(password) = matches.value_of("password") {
            if let Some(wallet_folder) = matches.value_of("folder") {
                print!("Creating new wallet \"{:}\" in directory \"{:}\"\n", wallet_name, wallet_folder);
                wallet_manager::Wallet::create(wallet_folder, wallet_name, password).unwrap(); 
            }       
            else {
                print!("Creating new wallet \"{:}\" in default directory \"./wallets\"\n", wallet_name);
                wallet_manager::Wallet::create("wallets", wallet_name, password).unwrap();
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