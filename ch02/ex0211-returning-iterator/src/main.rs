/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, Iteration, example 11, 
/// using the iter method to return an iterator
/// 
/// code examples and comments are taken from the book
/// 

fn main() {
    let needle = 0xCB;
    println!("needle: {}", needle);
    let haystack = [1, 1, 2, 5, 15, 52, 132, 203, 877, 4140, 21147];
    println!("haystack: {:?}", haystack);
    
    // calling iter() on the array returns an iterator on a reference
    for item in haystack.iter() {
        // once again the * before item dereferences the item to return its referent
        if *item == needle {
            println!("found it: {}", item);
        }
    }

    // some types allow to use iter_mut() and into_iter() methods
    // iter_mut() allows it that you change values while you iter over them
    // into_iter() is closer to iter() but returns elements by value
    // this prevents other parts of this code from accessing the values
    let needle_into = 0o204;
    println!("needle: {}", needle_into);

    for item in haystack.into_iter() {
        // dereferencing the item is no longer required
        if item == needle_into {
            println!("found it (into): {}", item);
        }
    }
    
}
