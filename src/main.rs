use std::io::{self};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the number guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {secret_number}");

    loop {
        println!("Enter a guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed {guess}");

        if make_guess(secret_number, guess) {
            break;
        }

    }
}

fn make_guess(secret_number: i32, guess: i32) -> bool {

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); return true; },
        }
    return false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_returns_true_when_correct() {
        let result = make_guess(2, 2);
        assert!(result);
    }
    #[test]
    fn guess_returns_false_when_incorrect() {
        // too large
        let result = make_guess(2, 3);
        assert!(!result);
        // too small
        let result = make_guess(2, 1);
        assert!(!result);
    }
}
