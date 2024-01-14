/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, Iteration, example 13, 
/// testing how fast your computer can increment a counter
/// 
/// code examples and comments are taken from the book
/// 
/// while -> looping until a condition changes its state
/// a while loop holds as long as a condition holds
/// a condition may be any expression that evaluates to true or false
/// 
/// bringing Duration and Instant types from std::time into local scope
use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    // create a duration that represents 1 second
    let time_limit = Duration::new(1,0);
    // access the systems clock time
    let start = Instant::now();
    
    // testing how fast your computer can increment a counter
    // continue to execute a block while a time limit has not been reached
    // (Instant::now() - start) -> an Instant minus an Instant returns a duration
    while (Instant::now() - start) < time_limit {
        // println!("{:?} -> {}", Instant::now(), count);
        count += 1;
    }

    println!("{}", count);
}
