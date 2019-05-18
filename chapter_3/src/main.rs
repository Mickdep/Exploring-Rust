const MAX_POINTS: u32 = 334;

fn main() {
    println!("Hello, chapter_3!");
    println!("Printing constant: {}", MAX_POINTS);
    let parsed_string: u32 = "344".parse().expect("That was not a string");
    println!("Parsed string: {}", parsed_string);
    let int8: i8 = 4;
    let int16: i16 = 100;
    let int32: i32 = 3344;
    let hex_value: u32 = 0x03;
    let binary_value: u32 = 0b1111000011110000;
    let byte_value: u8 = b'A';
    println!("Printing binary value: {}", binary_value);
    println!("Printing byte value: {}", byte_value);
    let float_value = 4344.0;
    let float_value_32: f32 = 3432.0;

    let my_tuple: (String, i32, bool) = (String::from("MyString"), 3448, true);
    println!("Printing tuple by destructuring using pattern matching: ");
    let (a, b, c) = &my_tuple;
    println!("Value a: {}, Value b: {}, Value c: {}", a, b, c);
    println!("Printing tuple by accessing the indices: ");
    print!("Value a: {} ", my_tuple.0);
    print!("Value b: {} ", my_tuple.1);
    print!("Value c: {} ", my_tuple.2);
    println!();
    let my_array = [34, 322, 500, 33];
    let my_array_with_type: [i32; 6] = [22, 2345, 2467, 556, 222, 7];

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("Printing y: {}", y);
    let val = weird_return();
    println!("weird_return() returned: {}", val);

    let condition = true;
    let variable = {
        if condition {
            5
        } else {
            20
        }
    };

    println!("Printing variable: {}", variable);

    do_loop();
    do_for_loop();
}

fn weird_return() -> i32 {
    println!("Inside weird_return");
    5
}

fn do_loop() {
    let mut counter: i8 = 0;
    loop {
        println!("Inside loop {}", counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }
}

fn do_for_loop(){
    let my_array: [i32; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    for element in my_array.iter(){
        println!("The value is {}", element);
    }

    for number in 1..5 {
        println!("Value is: {}" , number);
    }

    for number in (1..5).rev(){
        println!("Value is: {}", number);
    }
}
