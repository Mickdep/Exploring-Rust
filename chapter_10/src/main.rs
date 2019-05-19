use std::fmt;

struct Point<T> {
    x: T,
    y: T,
}

struct FlexiblePoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> FlexiblePoint<T, U>{
    fn print(&self){
        println!("This is the impl of a generic struct");
    }
}

impl FlexiblePoint<i32, i32>{
    fn print_concrete(&self){
        println!("This is the concrete impl of a generic struct");
        println!("Values: ({}, {})", self.x, self.y);
    }
}

pub trait Summary{
    fn summarize(&self) -> String;
}

pub struct Tweet{
    user: String,
    content: String
}

pub struct NewsArticle{
    headline: String,
    publisher: String,
    year: i32
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{} tweeted: {}", self.user, self.content)
    }
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{} published {} in {}", self.publisher, self.headline, self.year)
    }
}

fn main() {
    let number_vector: Vec<u32> =
        vec![123, 6, 642, 21245, 6343, 234, 666, 3745, 2, 234, 7, 991, 34];

    let number_array = [34, 226, 6545, 234, 234, 2222];
    print_biggest_number_in_list(number_vector);
    print_biggest_number_in_generic_list(&number_array);

    let integer_point = Point { x: 4, y: 5 };
    let float_point = Point { x: 3.0, y: 5.0 };
    let integer_and_float_point = FlexiblePoint { x: 5, y: 3.0 };
    integer_and_float_point.print();
    let integer_flexible_point = FlexiblePoint {x: 18, y: 30};
    integer_flexible_point.print_concrete();



    let news_article = NewsArticle{
        headline: String::from("Mick is awesome"),
        publisher: String::from("Myself"),
        year: 2019
    };

    let tweet = Tweet{
        user: String::from("@mick_him_self"),
        content: String::from("Mick is awesome")
    };

    let tweet_summary = tweet.summarize();
    let news_article_summary = news_article.summarize();
    println!("{}", tweet_summary);
    println!("{}", news_article_summary);
}

fn print_biggest_number_in_list(list: Vec<u32>) {
    let mut biggest_number: u32 = 0;
    for i in list {
        if i > biggest_number {
            biggest_number = i;
        }
    }

    println!("Biggest number in the list is {}", biggest_number);
}

fn print_biggest_number_in_generic_list(list: &[u32]) {
    let mut biggest_number: u32 = 0;
    for &i in list {
        if i > biggest_number {
            biggest_number = i;
        }
    }

    println!("Biggest number in the list is {}", biggest_number);
}

fn print_biggest_number_in_truly_generic_list<T>(list: &[T]) {
    // let mut biggest = list[0];

    // for &item in list {
    //     // if item > biggest {
    //     //     biggest = item;
    //     // }
    // }
}
