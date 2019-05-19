#[derive(Debug)]
enum EuroTypes{
    Text(String),
    Number(i32)
}


use std::collections::HashMap;

fn main() {
    //Creating a new empty vector of i32's
    {
        let mut vector: Vec<i32> = Vec::new();
        vector.push(35);
        vector.push(33);
        vector.push(234);
    } //vector goes out of scope here, vector and all it's elements are destroyed

    //Creating a new vector with values in it
    let new_vector = vec![1, 2, 3];

    let third_element_ref = &new_vector[2];
    println!("Third element in vector is {}", third_element_ref);
    let third_element = new_vector.get(2);

    match new_vector.get(2){
        Some(i) => println!("Third element is {}", i),
        None => println!("Nothing")
    }

    //Looping over vector
    for i in &new_vector{
        println!("Vector value is: {} ", i);
    }


    let eurotypes_vec = vec![EuroTypes::Number(35), EuroTypes::Text(String::from("Thirty-five"))];
    for i in &eurotypes_vec{
        println!("{:?}", i);
    }

    let string1 = String::from("Hello, ");
    let string2 = String::from("world");
    let string3 = String::from("!");
    let formatted_string = format!("{}{}{}", string1, string2, string3);
    println!("{}", formatted_string);


    create_and_print_hashmap();
}


fn create_and_print_hashmap(){
    let mut team_map: HashMap<String, u32> = HashMap::new();
    team_map.insert(String::from("Football"), 524);
    team_map.insert(String::from("Handball"), 56);
    team_map.insert(String::from("Hockey"), 774);

    for (key, value) in &team_map{
        println!("The {} team has {} points", key, value);
    }

    match team_map.get("Football"){
        Some(val) => println!("The football team has {} points", val),
        None => println!("Couldn't get value")
    }
}
