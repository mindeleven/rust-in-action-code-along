/// coding along with Rust in Action by Tim McNamara
/// Chapter 3, Compound Data Types, chapter 3.5, Defining and making use of enum 
/// 
/// code examples and comments are taken from the book
/// 

#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown
}

type Message = String;

fn parse_log(line: &'static String) -> (Event, Message) {
    
    let parts: Vec<&str> = line.splitn(2, ' ').collect();

    unimplemented!()
}

fn main() {
    println!("Hello, world!");
}
