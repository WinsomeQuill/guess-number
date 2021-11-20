use std::io::stdin;
use rand::Rng;

fn read_line() -> u32 {
    let mut buf = String::new();
    stdin().read_line(&mut buf);
    return match buf.trim_end().parse::<u32>() {
        Ok(i) => i,
        Err(_) => 0,
    };
}

fn main() {
    let mut attempts: u32 = 0;
    let number: u32 = rand::thread_rng().gen_range(1, 10);
    loop {
        
        let line = read_line();
        let res = match line {
            0 => { println!("Invalid number!"); continue; },
            _ => line
        };

        if number == res { println!("Congratulations! Number: {} | Attempts: {}", number, attempts); break; }
        println!("It's not my number!"); attempts += 1;
    }
}
