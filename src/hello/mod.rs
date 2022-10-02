pub fn doprint() {
    println!("Apparently this works");
}

use std::fs::File;

pub fn open_hello_file() -> File {
    File::open("hello.txt").unwrap()
}

pub fn open_hello_file2() -> File {
    File::open("hello.txt").expect("Ei onnaa")
}
use std::io;
use std::io::Read;

pub fn read_line(f: &mut File) -> Result<String, io::Error> {
    let mut s = String::new();
    f.read_to_string(&mut s)?; // Return Err if needed
    Ok(s)
}
