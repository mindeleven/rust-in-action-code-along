/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, chapter 2.8,
/// advanced function definitions: explicit lifetime annotations, 
/// 
/// code examples and comments are taken from the book
/// 
/// lifetime annotations
/// -> provides more specific information to the Rust compiler 
///    about data that lives outside the function
/// -> functions that use references have data that exists outside of their scope
/// -> Rust wants to know if the whether the referent data should outlive the function call
///    or be cleaned up when the function returns
/// 
/// the lifetime system usually works unaided (lifetime elision)
/// the compiler needs assistance in difficult cases
/// -> when multiple references are accepted as arguments
/// -> when a reference is returned from a function

use std::ops::Add;
use std::time::Duration;

fn main() {
    let a = 10;
    let b = 20;
    // reference to 10 and reference to 20 is passed
    // no lifetime notation is required when calling the function
    let res = add_with_lifetimes(&a, &b);
    println!("result: {}", res);

    let floats = add(1.2, 3.4);
    let ints = add(60, 10);
    // calling add with duration values that represent a duration between two points in time
    let duration = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("result floats: {}", floats);
    println!("result ints: {}", ints);
    println!("result duration: {:?}", duration);
}

#[allow(dead_code)]
// a function signature that includes explicit lifetime annotations
// &'a i32 reads as reads as "reference to an i32 with lifetime a"
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    // the * idicates that we're adding the values referred to by i and j
    // rather than adding the reference directly
    *i + *j
}

/// Generic funtions
/// -> functions that can handle many possible input types
/// expample: a function that can be called by many input types (as long as they are all the same)
/// T is a type variable
/// capital letters in place of a type indicate a generic type
/// conventionelly U, U and V are used
/// only a subset of all types implement addition
/// when creating generic functions that include an operator
/// that operator's trait must be included as a trait bound
#[allow(dead_code)]
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}