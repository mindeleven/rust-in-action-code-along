/// coding along with Rust in Action by Tim McNamara
/// Chapter 1, Introducing Rust, example 5, buffer overflow
/// code examples and comments are taken from the book
/// 
/// Rust programs avoid buffer overflow
/// buffer overflow for example means that you're attempting to access 
/// the 12th element of an array that has only 6 elements
/// example: attempting to create a buffer overflow
/// 
/// THIS EXAMPLE DOES NOT COMPILE
/// 
fn main() {
    let fruit = vec!['🍇', '🍉', '🥝'];
    
    // now, attempt to access the 5th element of fruit
    // the compiler will complain about the following line:
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4'
    let buffer_overflow = fruit[4];
    
    // assert_eq! allows programmers to include assertions
    // he two sides of assert_eq! must evaluate to equal, otherwise the program will panic
    assert_eq!(buffer_overflow, '🍍');
}
