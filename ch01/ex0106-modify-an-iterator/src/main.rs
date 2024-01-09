/// coding along with Rust in Action by Tim McNamara
/// Chapter 1, Introducing Rust, example 6, modify an iterator
/// 
/// attempting to modify an iterator while iterating over it
/// 
/// code examples and comments are taken from the book
/// 
/// Rust programs avoid “iterator invalidation”
/// iterator invalidation describes an issue caused by something 
///  that is being iterated over being altered mid-way through
/// 
/// THIS EXAMPLE DOES NOT COMPILE
/// 
fn main() {
    // initializing a mutable vector
    let mut letters = vec![
        'a', 'b', 'b'
    ];
    
    // iterrating over letters
    for letter in letters {
        println!("{}", letter);
        // creating a new copy of letter and appending it to the end of letters
        letters.push(letter.clone());

        // code will not compile !!!
    }
}
