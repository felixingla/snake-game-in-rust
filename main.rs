use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("→ Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut number_of_guesses = 1;

    loop {
        println!("→ Please input your guess:");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("→ That's not a number");
                continue;
            },
        };

        println!("→ Your guess: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("→ {} is too small, try again!", guess);
            },
            Ordering::Greater => {
                println!("→ {} is too big, try again!", guess);
            },
            Ordering::Equal => {
                println!("→ {} is the number, you win!", guess);
                println!("→ The total number of guesses was {}", &number_of_guesses);
                break;
            }
        }

        number_of_guesses = *&mut number_of_guesses + 1;
    }

}