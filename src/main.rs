mod hello;

use std::fs::File;
use std::io::ErrorKind;

fn open_hello_file() -> File {
    match File::open("hello.txt") {
        Ok(x) => x,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("No hello!"),
            other_error => panic!("{:?}", other_error),
        },
    }
}

fn main() {
    crate::hello::doprint();
    println!("Hello, world!");

    let f = open_hello_file();
    panic!("Good bye, cruel world!");
}
