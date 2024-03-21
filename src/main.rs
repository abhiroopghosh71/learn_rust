use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input the number you guessed");

        let mut guess = String::new(); // mut (mutable) means the guess can be changed later

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Correct answer: {secret_number}");
                break;
            }
        }
    }
}
