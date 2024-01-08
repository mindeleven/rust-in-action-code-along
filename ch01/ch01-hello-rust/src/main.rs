/// coding along with Rust in Action by Tim McNamara
/// Chapter 1, Introducing Rust, example 1, hello world
/// code examples and comments are taken from the book
/// 
fn main() {
    greet_world();
}

fn greet_world() {
    println!("Hello, world!");

    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";

    let regions = [southern_germany, japan];

    for region in regions.iter() {
        println!("{}", &region);
    }
}