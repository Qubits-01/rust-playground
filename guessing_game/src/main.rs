use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // [ PROCESSING A GUESS ]
    println!("Guess the number!");

    // [ GENERATING A RANDOM NUMBER ]
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // [ ALLOWING MULTIPLE GUESSES WITH LOOPING ]
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // [ STORING VALUES WITH VARIABLES ]
        let mut guess: String = String::new();

        // [ RECEIVING USER INPUT ]
        // [ HANDLING POTENTIAL FAILURE WITH RESULT ]
        // [ HANDLING INVALID INPUT ]
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // [ PRINTING VALUES WITH println! PLACEHOLDERS ]

        println!("You guessed: {}", guess);

        // [ COMPARING THE GUESS TO THE SECRET NUMBER ]
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // [ QUITTING AFTER A CORRECT GUESS ]
                println!("You win!");
                break;
            }
        }
    }
}
