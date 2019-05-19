struct User {
    username: String,
    password: String,
    age: u8,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, world!");

    let user1 = register_user(String::from("Mick"), String::from("SuperSecret"), 86);
    let user2 = register_user_simple(String::from("OtherMick"), String::from("SuperSecret2"), 32);

    println!(
        "User1: {}\t{}\t{}\t{}",
        user1.username, user1.password, user1.age, user1.active
    );
    println!(
        "User2: {}\t{}\t{}\t{}",
        user2.username, user2.password, user2.age, user2.active
    );

    let updated_user1 = create_new_user_from_user(
        user1,
        String::from("UpdatedUsername"),
        String::from("UpdatedPassword"),
    );
    println!(
        "User2: {}\t{}\t{}\t{}",
        updated_user1.username, updated_user1.password, updated_user1.age, updated_user1.active
    );

    let my_color = Color(234, 255, 123);
    let my_point = Point(22, 235, 66);

    //Rectangles project here
    let rectangle = Rectangle {
        width: 523,
        height: 29287,
    };

    let area: u32 = calculate_area(&rectangle);
    println!("Area of the rectangle {:?} is: {}", rectangle, area);
}

fn register_user(username: String, password: String, age: u8) -> User {
    User {
        username: username,
        password: password,
        age: age,
        active: true,
    }
}

fn register_user_simple(username: String, password: String, age: u8) -> User {
    User {
        username,
        password,
        age,
        active: true,
    }
}

fn create_new_user_from_user(user: User, new_username: String, new_password: String) -> User {
    User {
        username: new_username,
        password: new_password,
        ..user
    }
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
