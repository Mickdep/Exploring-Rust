use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to my guesing game.");
    loop {
        println!("Input a string of 3 characters to guess: ");

        let mut guess = get_user_input();

        if guess == "q"{
            println!("Shutting down...");
            break;
        }
        //  ^^^ mut keyword to make this variable mutable
        //In this case it's there to actually assign it from read_line

        while guess.len() != 3 {
            println!("Please enter a string of 3 characters");
            guess.clear();
            guess = get_user_input();
        }

        //Pass the guess as so we transfer ownership to check_guess, which then frees that memory when it goes out of scope
        check_guess(guess);

        println!("Please enter a number to guess: ");
        let guess: u32 = match get_user_input()
            .trim()
            .parse(){
                Ok(guess) => guess,
                Err(_) => continue,
            };
        check_guess_number(guess);
    }
}

//Takes String as argument so it takes ownership of the string
fn check_guess(guess: String) {
    let value_to_guess = "adj";
    println!("Checking guess: {}", guess);
    if guess == value_to_guess {
        println!("You guessed correctly!");
    } else {
        println!("You guessed incorrectly!");
    }
}

fn get_user_input() -> String {
    let mut input = String::new(); //Create string to store input in
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line."); //Read input and store it in input

    return String::from(input.trim_end()); //Trim the trailing \r\n
}

fn check_guess_number(guess: u32) {
    let random_number = rand::thread_rng().gen_range(0, 101);
    println!("Guessing: {}", guess);
    if random_number == guess {
        println!("You guessed it correctly");
    } else {
        println!(
            "You guessed it incorrectly! The number was {}",
            random_number
        );
    }
}
