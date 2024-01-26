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
use regex::Regex;
use clap::{App,Arg};

fn main() {
    // creating a File requires a path argument and error handling
    /* 
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
    */
    // incrementally build up a command argument parser
    // each argument takes an Arg
    // cargo run -- <pattern>
    // example: cargo run -- fever
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true))
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    
    println!("pattern from command line: {}", re.as_str());


    // alternate approach: reading a file line by line via BufReader::lines()
    println!("=> alternate approach: reading a file line by line via BufReader::lines()");
    let f_2 = File::open("./src/readme.txt").unwrap();
    let reader_2 = BufReader::new(f_2);
    // BufReader::lines() removes the trailing newline character from each line
    for line_ in reader_2.lines() {
        // unwrapping the Result at each line is still required
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
