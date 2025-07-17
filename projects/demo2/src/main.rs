fn main() {
    println!("Please guess the number");

    println!("Please input your guess");

    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    println!("You guessed {str}");
}
