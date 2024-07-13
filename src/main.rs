use std::io;
use std::cmp::Odering;

fn main() {
    println!("Guess the number");

    println!("Please input your guess");
    
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess).expect("Failed to read 1 line");
    println!("You guessed : {}", guess);
}
