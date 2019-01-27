#[macro_use]
extern crate serde_derive;
extern crate serde_json; 


extern crate nat_messaging;

use std::io::{self, Write, Read};
use std::fs::File;
use std::process;

mod wallet_manager;

fn main() -> io::Result<()>{
    let mut wm = wallet_manager::WalletManager::new();

    let mut pass = String::new();
    print!("\n"); 
    std::io::stdin().read_line(&mut pass)?;

    let a = wm.open_wallet(&"test2.txt".to_string(), &pass).unwrap();
    print!("{:?}\n", a);
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
