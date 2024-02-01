#![allow(dead_code)]
/// coding along with Rust in Action by Tim McNamara
/// Chapter 3, Compound Data Types, chapter 3.6.2,
/// implementing STD::FMT::DISPLAY for your own types 
/// 
/// code examples and comments are taken from the book
/// 
/// -> code snippet to implement Display for a struct 
/// that includes fields that also need to implement Display

/// bringing the std::fmt crate into local scope allows to make use of fmt::Result
/// bringing Display into local scope avoids the need for us to prefix it as fmt::Display

use std::fmt;
use std::fmt::Display;

// struct File and enum FileState like in previous example
// an enum’s variants are assumed to be public if the overall type is made public
#[derive(Debug,PartialEq)]
pub enum FileState {
    Open,
    Closed,
}
#[derive(Debug)]
pub struct File {
    pub name: String,
    // struct is public but File.data remains private 
    // if a third party were to import this crate via use
    data: Vec<u8>,
    pub state: FileState,
}

/// implementing the Display trait for FileState
impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // use of write! to do the work for us
            // Strings already implement Display themselves
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// implementing the Display trait for File
impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // relying on the FileState Display implementation in our own code
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    // even though the File struct is public
    // its methods must be explicitly marked as such too
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    let f6 = File::new("f6.txt");
    // the Debug implementation prints out a familiar message
    println!("{:?}", f6);
    // Display implementation follows its own rules
    // displaying itself as <f6.txt (CLOSED)>
    println!("{}", f6);
}
