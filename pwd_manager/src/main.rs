#[macro_use(println_stderr)]
extern crate nat_messaging;

use std::io::{self, Write, Read};
use std::fs::File;
use std::process;

fn main() -> io::Result<()>{
    let mut i = 0;
    loop {
        let message = match nat_messaging::read_stdin() {
            Ok(m) => m,
            Err(_) => process::exit(1),
        };
        //println_stderr!("received {}", message);
        let answer = format!("\"{} {}\"", i, "Bonjour");
        nat_messaging::write_stdout(&answer);
        i += 1;
    }
}
