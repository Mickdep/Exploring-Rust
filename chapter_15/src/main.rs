use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List{
    Cons(i32, Box<List>),
    Nil
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T{
        &self.0
    }
}

struct CustomSmartPointer{
    data: String
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn main() {
    println!("Hello, world!");
    let b = Box::new(5); //Store an i32 on the heap

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    dereference_operator();
    hello(&MyBox::new(String::from("Mick")));
    hello(&Box::new(String::from("Mick")));

    let smart_ptr1 = CustomSmartPointer{ data: String::from("smart_ptr1 here!")};
    { //Begin of scope
        //Will get 'drop' function called first because it goes out of scope first.
        drop(smart_ptr1); //Custom drop
        let smart_ptr2 = CustomSmartPointer{ data: String::from("smart_ptr2 here!")};
    }//End of scope
} //End of scope for smart_ptr1

fn dereference_operator(){
    let print_closure = ||{
        println!("y and x are equal")
    };

    let x = 5;
    let y = &x;
    // vv use the dereference operator here to get value from the reference pointer.
    if *y == x{
        print_closure();
    }

    let x = 5;
    let y = MyBox::new(5);
    if *y == x{
        print_closure();
    }
}

fn try_compare(value: &String){
    let other_value = String::from("MyString");

    //NOTE: This doesn't work because you can't compare a reference type with a String
    // if other_value == value{
    //     println!("Are equal");
    // }

    if other_value == *value{
        println!("Are equal");
    }
}

fn hello(name: &str){
    println!("Hello {}", name);
}
