
/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, Iteration, example 10, 
/// searching the needle in a haystack
/// 
/// code examples and comments are taken from the book
/// 

fn main() {
    let needle = 0o52;
    println!("needle: {}", needle);
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    println!("haystack: {:?}", haystack);
    
    // ampersand (&) at haystack acts as an unary operator
    // that returns a reference to haystack (read-only reference)
    // references to arrays are useful to iterate to a reference of the elements
    // the haystack is "borrowed"
    for item in &haystack {
        // the * before item dereferences the item to return its referent
        // otherwise the compiler complains:
        // error[E0277]: can't compare `&{integer}` with `{integer}`
        if *item == needle {
            println!("found it: {}", item);
        }
    }
}
