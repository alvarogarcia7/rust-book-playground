use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn read_number() -> i32 {
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    loop {
        let guess = read_number();

        let (should_break, message) = match guess.cmp(&secret_number) {
            Ordering::Less => (false, "Too small!"),
            Ordering::Greater => (false, "Too big!"),
            Ordering::Equal => (true, "You win!"),
        };

        println!("{}", message);

        if should_break {
            break;
        }
    }
}
