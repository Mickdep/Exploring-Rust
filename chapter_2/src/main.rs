use std::io;
fn main() {
    println!("Welcome to my guesing game.");
    println!("Input a string of 3 characters to guess: ");

    let mut guess = get_user_input();
    //  ^^^ mut keyword to make this variable mutable
    //In this case it's there to actually assign it from read_line
    
    while guess.len() != 3 {
        println!("Please enter a maximum of 3 characters");
        guess = get_user_input();
    }

    //Passes the guess String to check_guess(), thus transferring ownership. check_guess() frees the memory.
    check_guess(guess);
}

//Takes String as argument so it takes ownership of the string
fn check_guess(guess: String){
    let value_to_guess = "adj";
    println!("Checking guess: {}", guess);
    if guess == value_to_guess {
        println!("You guessed correctly!");
    }else{
        println!("You guessed incorrectly! Shutting down...");
    }
}

fn get_user_input() -> String{
    let mut input = String::new(); //Create string to store input in
    io::stdin().read_line(&mut input).expect("Failed to read line."); //Read input and store it in input

    return String::from(input.trim_end()); //Trim the trailing \r\n
}