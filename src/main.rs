mod hello;

fn main() {
    crate::hello::doprint();
    println!("Hello, world!");

    let mut f = hello::open_hello_file2();
    let s = hello::read_line(&mut f).unwrap();
    panic!("Good bye, cruel world! '{}' {:?}", s.trim(), f);
}
