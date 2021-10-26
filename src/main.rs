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

fn clear_attempts() {
    unsafe {
        ATTEMPTS = 0;
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

fn read_line() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input);
    return input;
}

fn main() {
    let mut run: bool = true;
    generate_number();
    while run {
        println!("Write any number of 1 to 10: ");
        
        let input = read_line();
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
            println!("Maybe restart game? [Y/n]");
            loop {
                let mut input: String = read_line();
                input = input.trim().to_lowercase();
                if input != "n" && input != "y" {
                    println!("Error, use Y or N!");
                    continue;
                }

                if input == "n" {
                    run = false;
                    break;
                } else {
                    clear_attempts();
                    generate_number();
                    break;
                }
            }
        }
        
        if input != get_number() {
            println!("No, this is not my number!");
        }
    }
}
