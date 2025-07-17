use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Please guess the number");

    let secret_number = rand::rng().random_range(1..=100);

    println!("Secret number id {secret_number}");

    loop {
        println!("Please enter the number");

        let mut guess = String::new(); // Mutable veriable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess // Shadowing the variable and pattern matching
            .trim()
            .parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            // Pattern matching
            Ordering::Less => println!("Less"),
            Ordering::Equal => {
                println!("You're right!!!");
                break;
            }
            Ordering::Greater => println!("Greater"),
        }

        println!("You guess the number {guess}");
    }
}
