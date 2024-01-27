/// coding along with Rust in Action by Tim McNamara
/// Chapter 3, Compound Data Types, chapter 3.1, 
/// Using plain functions to experiment with an API
/// 
/// code examples and comments are taken from the book
/// 
/// relaxing compiler warnings while working through ideas
#[allow(unused_variables)]

/// creating a type alias
type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

/// relaxing a compiler warning about an unused function
/// using ! as a return type indicates to the Rust compiler that this function never returns
/// ! is known as the “Never” type
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    // File "inherits" all of String’s methods
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(&mut f1, vec![]);
    close(&mut f1);
}
