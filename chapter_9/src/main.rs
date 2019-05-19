use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;

fn main() {
    open_or_create_file();
    let result = read_content_from_file(String::from("hello.txt")).expect("Could not read content");
    println!("Read content from file: {}", result);
    let result2 = read_content_from_file_with_questionmark(String::from("hello.txt")).unwrap();
    println!("Read content from file: {}", result2);
    let result3 = read_content_from_file_shortest("hello.txt").unwrap();
    println!("Read content from file: {}", result3);

}

fn open_or_create_file() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem!"),
            },
            other_error => panic!("There was a problem opening the file"),
        },
    };
}

fn read_content_from_file(filename: String) -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut content = String::new();

    //This Result is returned
    match f.read_to_string(&mut content){
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

fn read_content_from_file_with_questionmark(filename: String) -> Result<String, io::Error>{
    let mut content = String::new();

    File::open(filename)?.read_to_string(&mut content)?;

    Ok(content)
}

fn read_content_from_file_shortest(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}