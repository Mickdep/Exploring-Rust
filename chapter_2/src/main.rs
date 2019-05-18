use std::io;

fn main() {
    println!("Welcome to my guesing game.");
    println!("Input a number to guess: ");

    let mut guess = String::new();
    //  ^^^ mut keyword to make this variable mutable

    io::stdin().read_line(&mut guess).expect("Failed to read line.");

    println!("You guessed {}", guess);

}
