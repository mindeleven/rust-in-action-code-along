/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, chapter 2.13, 
/// reading from files
/// 
/// code examples and comments are taken from the book
/// 
/// task at hand: modifying the grep-lite tool from previous examples
/// to be able to search within files
/// 
/// general pattern is to open a file object, the wrap it in BufReader
/// 

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // creating a File requires a path argument and error handling
    let f = File::open("./src/readme.txt").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    loop {
        // reading from disk can fail and we need to explicitly handle this
        // in this case, errors crash the program
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break
        }

        println!("{} ({} bytes long)", line, len);
        
        // shrink the string back to length 0
        // preventing lines from persisting into the following ones
        line.truncate(0);
    }
}
