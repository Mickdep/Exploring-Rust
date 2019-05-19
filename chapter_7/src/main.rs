mod user;

fn main() {
    println!("[User registration]");
    let first_user = user::User::new(String::from("Mick"), String::from("Secret"), 42);
    let user_is_authorized = first_user.is_authorized("Secret");
    println!("User {} authorized: {}", first_user.name, user_is_authorized);
}

