use std::env;

fn main() {
    println!("Hello, world!");
    for arg in env::args().skip(1) {
        println!("{}", arg);
    }
}
