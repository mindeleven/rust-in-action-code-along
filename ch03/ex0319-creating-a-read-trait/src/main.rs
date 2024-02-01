#![allow(unused_variables)]
/// coding along with Rust in Action by Tim McNamara
/// Chapter 3, Compound Data Types, chapter 3.6.1, creating a read trait, 
/// 
/// code examples and comments are taken from the book
/// 
/// -> defining the bare bones of a Read trait for File
/// -> showing the distinction between the trait and the impl keyword
///    impl for trait is used for attaching a trait to a specific type
/// 

#[derive(Debug)]
/// defining a stub File type
struct File;

/// providing a specific name for the trait
trait Read {
    // a trait block includes a type signatures that implementors must comply with
    // the pseudotype Self is a placeholder for the type that will eventually be implementing Read
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        // returning a stub value that complies with the type signature required
        Ok(0)
    }
}

fn main() {
    let f = File{};
    let mut buffer = vec!();
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}
