use rand;
use std::io;

fn main() {
    println!("guess the number");
    println!("Input:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("You guessed: {}", guess);
}
