/// coding along with Rust in Action by Tim McNamara
/// Chapter 3, Compound Data Types, chapter 3.4, Returning errors 
/// 
/// code examples and comments are taken from the book
/// 
/// >>> making use of the result return type
/// Rust’s approach to error handling is to use a type 
/// that stands for both the standard case and the error case (Result type)
/// Result has two states, Ok and Err
/// -> functions that interact with the file system return Result<T, String>
/// T is the intended type and String is used to report back error messages
/// when we call those functions, there is now an additional call to unwrap() which unwraps Ok(T)
/// -> Result is an enum defined in Rust’s standard library

/// bringing the rand crate into this crate’s scope
extern crate rand;
/// bringing the random number generator trait into scope
use rand::Rng;

/// helper function to enable us to trigger sporadic errors
fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_weighted_bool(n)
}
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        // stylistic change to shorten the code block
        File { name: String::from(name), data: Vec::new() }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    
    // read() returns Result<T, E>
    // where T is an integer of type usize and E is String
    // using String allows to provide arbitrary error messages
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        // wrapping read_length in Ok because the function returns a Result
        Ok(read_length)
    }
}

/// open() and close() now take full ownership of their File arguments
/// it enables the File argument to be inserted into Ok(T) as T then returned
fn open(f: File) -> Result<File, String> {
    // provoking an error case: once in 10,000 executions, return an error
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

/// open() and close() now take full ownership of their File arguments
/// it enables the File argument to be inserted into Ok(T) as T then returned
fn close(f: File) -> Result<File, String> {
    // provoking an error case: once in 100,000 executions, return an error
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}


fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];
    
    // re-assigning the value from the result of the calls to open() and close() 
    // allows the variable f4 to re-claim ownership of its File value
    // unwrap() unwraps T from Ok, leaving T
    // calling Result.unwrap() is considered poor style 
    // because if effectively ignors the possibility of errors
    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    
    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}
