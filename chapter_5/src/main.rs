struct User {
    username: String,
    password: String,
    age: u8,
    active: bool,
}

impl User {
    fn are_equal(&self, other: &User) -> bool {
        self.username == other.username
            && self.password == other.password
            && self.age == other.age
            && self.active == other.active
    }

    fn is_authorized(&self, password: String) -> bool {
        self.password == password
    }

    fn new() -> User {
        User {
            username: String::from("Default"),
            password: String::from("DefaulPass"),
            age: 0,
            active: true,
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }
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

    let area: u32 = calculate_area_ext(&rectangle);
    println!("Area of the rectangle {:?} is: {}", rectangle, area);

    println!(
        "Area of the rectangle {:?} by calling its method is {}",
        rectangle,
        rectangle.calculate_area()
    );

    //User implementation code
    let my_user = User {
        username: String::from("MyUser"),
        password: String::from("MyPassword"),
        age: 25,
        active: true,
    };

    let my_user2 = User {
        username: String::from("MyUser2"),
        password: String::from("Mypassword"),
        age: 56,
        active: false,
    };

    println!(
        "Checking if user {} can authenticate with password {}...Result is: {}",
        my_user.username,
        "MyPassword",
        my_user.is_authorized(String::from("MyPassword"))
    );
    println!(
        "Checking user {} and user {} are equal: {}",
        my_user.username,
        my_user2.username,
        my_user.are_equal(&my_user2)
    );


    let newest_user = User::new();
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

fn calculate_area_ext(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
