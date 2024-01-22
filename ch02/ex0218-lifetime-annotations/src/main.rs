/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, chapter 2.8,
/// advanced function definitions: explicit lifetime annotations, 
/// 
/// code examples and comments are taken from the book
/// 

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
// a function signature that includes explicit lifetime annotations
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    unimplemented!()
}
