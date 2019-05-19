use std::io;

#[derive(Debug)]
enum Food {
    Kebab,
    Hamburger,
    Pizza,
    Taco,
}

struct Customer {
    name: String,
    favorite_food: Food,
    visit_count: u32,
}

impl Customer {
    fn register_visit(&mut self) {
        self.visit_count += 1
    }

    fn set_favorite_food(&mut self, favorite_food: Food) {
        self.favorite_food = favorite_food
    }
}

fn main() {
    let mut customer = Customer {
        name: String::from("Mick"),
        favorite_food: Food::Kebab,
        visit_count: 0,
    };

    println!("Hello, welcome {}", customer.name);
    println!("Your favorite food is {:?}", customer.favorite_food);
    println!("You have visited this place {} times", customer.visit_count);
    println!("Registering your visit...");
    customer.register_visit();
    println!("You have visited this place {} times", customer.visit_count);
    println!("Now changing your favorite food from Kebab to Taco...");
    customer.set_favorite_food(Food::Taco);
    println!("Your favorite food is now: {:?}", customer.favorite_food);
    println!("Press any key to exit...");

    let test = Some("yee");
    let test2: Option<i32> = None;

    match_food(customer.favorite_food);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
}

fn match_food(food: Food) {
    match food {
        Food::Kebab => println!("You like kebab!"),
        Food::Taco => println!("You like taco!"),
        Food::Pizza => println!("You like pizza!"),
        Food::Hamburger => println!("You like hamburger!"),
    }
}
