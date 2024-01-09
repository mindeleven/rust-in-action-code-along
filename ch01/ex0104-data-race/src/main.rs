/// coding along with Rust in Action by Tim McNamara
/// Chapter 1, Introducing Rust, example 4, data race
/// code examples and comments are taken from the book
/// 
/// Rust programs avoid data races
/// data races mean that it's unable to determine how a program will behave 
/// from run to run because external factors are changing
/// example: attempting to create a data race
/// 
/// THIS EXAMPLE DOES NOT COMPILE
/// 
/// importing the standard library’s threading implementation
use std::thread;

fn main() {
    let mut data = 100;
    
    // this code is not deterministic
    // it’s impossible to know what data will be when main() exits
    // thread::spawn() takes a closure as an argument
    thread::spawn(|| { data = 500; });
    thread::spawn(|| { data = 1000; });

    // the compiler will complain very much about the two lines of code above
    // error[E0499]: cannot borrow `data` as mutable more than once at a time
    // error[E0502]: cannot borrow `data` as immutable because it is also borrowed as mutable
    // Rust will not allow multiple places of an application to have write access to data

    println!("{}", data); // immutable borrow occurs here

    // Some errors have detailed explanations: E0373, E0499, E0502.
    // For more information about an error, try `rustc --explain E0373`.

}
