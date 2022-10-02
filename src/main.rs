mod hello;

fn main() {
    crate::hello::doprint();
    println!("Hello, world!");

    let mut f = hello::open_hello_file2();
    let s = hello::read_line(&mut f).unwrap();
    println!("Good bye, cruel world! '{}' {:?}", s.trim(), f);

    let a = String::from("Big");
    let longger_ab;

    let b = String::from("Smaller");

    longger_ab = hello::longest(a.as_str(), b.as_str());

    println!("Big or smaller: {:?}", longger_ab);
    hello::longest_with_an_announcement(&a, &b, "Julistus!");
}
