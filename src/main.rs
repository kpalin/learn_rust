mod hello;

use std::fs::File;

fn open_hello_file() -> File {
    match File::open("hello.txt") {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    }
}

fn main() {
    crate::hello::doprint();
    println!("Hello, world!");

    let f = open_hello_file();
    panic!("Good bye, cruel world!");
}
