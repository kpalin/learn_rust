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

// Lifetimes  (Return value has the lifetime of the shortest lifetime of x and y)
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;
// Chapter 10: Generics and lifetimes:
pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
