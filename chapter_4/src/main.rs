fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = x;

    let my_string: String = String::from("Testing");
    println!("{}", my_string);
    let mut my_string2 = my_string.clone();
    my_string2.push_str(", Yeettt");
    println!("{}", my_string2);

    let my_string = String::from("MyString");
    takes_ownership(my_string);

    let my_string2 = String::from("MyString2");
    let give_back = takes_ownership_and_gives_back(my_string2);
    println!("{}", give_back);

    let my_string_ref = String::from("MyRef");
    let length = takes_reference(&my_string_ref);
    println!("Because we passed a reference we can still use my_string_ref here {} with length {}", my_string_ref, length);

    let mut changing_string = String::from("This is my string");
    changes_reference(&mut changing_string);
    println!("This is the string after: {}", changing_string);

    let string_to_slice = String::from("Hello world, this is a string to slice");
    let first_slice = &string_to_slice[0..6];
    let second_slice = &string_to_slice[6..];
    println!("First slice: {}, Second slice: {}", first_slice, second_slice);
}

fn takes_ownership(value: String){
    println!("Method has ownership of value {}", value);
}

fn takes_ownership_and_gives_back(value: String) -> String{

    value
}

fn takes_reference(value: &String) -> usize{
    value.len()
}

fn changes_reference(value: &mut String){
    value.push_str(", pushing value");
}
 