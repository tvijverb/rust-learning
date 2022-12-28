// some rust functions return Result Enum type
// can hold Ok and Err state
// we need to handle each of these for rust
// to compile and run
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    // file open returns Ok(content) or Error(errortype)
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // not found? no problem, try to create new
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e), // creating new file failed as well, abort abort
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },// print the error to std and quit
    };

    // same logic but more concise
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // .unwrap() unwraps Ok value or implicitly calls panic! when not Ok
    let greeting_file = File::open("hello.txt").unwrap();

    // .expect() unwraps Ok value but allows us to customize the error message
    let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
}

// long way to read file and contents and return the result or error
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// short way of reading a file
fn read_username_from_file_short() -> Result<String, io::Error> {
    // question mark ? operator can only be used if Result type
    // is used as function return type.
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// even more concise
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// shortest way to read file and return contents
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// .next()? returns Option<char> could return None
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

use std::error::Error;
// box error (any type of error)
fn main2() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

// user input, create a new type with additional checks
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
