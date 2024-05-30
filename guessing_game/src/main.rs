use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please Input your number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // shadowing previous value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;},
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
            Ordering::Greater => println!("Too Big"),
        }
    }
}
