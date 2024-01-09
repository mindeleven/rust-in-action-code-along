/// coding along with Rust in Action by Tim McNamara
/// Chapter 1, Introducing Rust, example 3, dangling pointers
/// code examples and comments are taken from the book
/// 
/// Rust programs are free from dangling pointers
/// dangling pointers are live references to data 
/// that has become invalid over the course of the program
/// example: attempting to create a dangling pointer
/// 
/// THIS EXAMPLE DOES NOT COMPILE
/// 
/// derive(Debug) annotation allows the enum to be printed by the println! macro
#[derive(Debug)]
#[allow(dead_code)]
enum Cereal {
    Barley, 
    Millet, 
    Rice, 
    Rye, 
    Spelt, 
    Wheat,
}
fn main() {
    // initializing an empty vector of Cereal
    let mut grains: Vec<Cereal> = vec![];
    // adding items to the grains vector
    grains.push(Cereal::Barley);
    grains.push(Cereal::Millet);
    grains.push(Cereal::Rye);
    grains.push(Cereal::Spelt);
    // delete grains and its contents
    drop(grains); // value moved here
    // attempt to access the deleted value
    // which will not compile
    println!("{:?}", grains); // value borrowed here after move
    // compiler not happy
}
