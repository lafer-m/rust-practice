use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input the guess number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed {}", guess);
        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
            Ordering::Greater => println!("too big"),
        }
    }
}