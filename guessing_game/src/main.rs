use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number");
    println!("Input:");
    let mut guess = String::new();
    let random = rand::thread_rng().gen_range(1..101);
    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&random) {
            Ordering::Less => println!("Too small {}", random),
            Ordering::Greater => println!("Too big {}", random),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
