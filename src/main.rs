pub mod hello {
    pub fn doprint() {
        println!("Test");
    }
}

fn main() {
    crate::hello::doprint();
    println!("Hello, world!");
}
