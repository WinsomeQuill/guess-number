extern crate rand;

use std::io;
use rand::Rng;

static mut NUMBER: u32 = 0;
static mut ATTEMPTS: u32 = 0;

fn get_attempts() -> u32 {
    unsafe {
        return ATTEMPTS;
    }
}

fn add_attempts() {
    unsafe {
        ATTEMPTS += 1;
    }
}

fn get_number() -> u32 {
    unsafe {
        return NUMBER;
    }
}

fn generate_number() {
    unsafe {
        NUMBER = rand::thread_rng().gen_range(1, 10);
    }
}

fn main() {
    generate_number();
    loop {
        println!("Write any number of 1 to 10: ");
        let mut input: String = String::new();
        
        io::stdin().read_line(&mut input)
            .expect("Error, invalid number!");
    
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error, invalid number!");
                continue;
            },
        };
        add_attempts();

        if input == get_number() {
            println!("Congratulations! You are win!\nNumber: {}\nTotal attempts: {}", input, get_attempts());
            break;
        }
        
        if input != get_number() {
            println!("No, this is not my number!");
        }
    }
}
