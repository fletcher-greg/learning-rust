extern crate rand;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("guess the number");
    let secret_num = thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter your guess");
        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Faile to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("Toobig"),
            Ordering::Equal => {
                println!("Yes! Congrats!");
                break;
            }
        }
    }
}
