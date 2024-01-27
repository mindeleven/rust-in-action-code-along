/// coding along with Rust in Action by Tim McNamara
/// Chapter 3, Compound Data Types, chapter 3.1, 
/// Using plain functions to experiment with an API
/// 
/// code examples and comments are taken from the book
/// 
/// relaxing compiler warnings while working through ideas
#[allow(unused_variables)]

/// modelling files with a struct
/// creating a persistent object that could represent a file
/// #[derive(Debug)] allows the println! macro to print a struct
#[derive(Debug)]
struct File {
    name: String,
    // using Vec<u8> allows for dynamic sizing
    data: Vec<u8>,
}

/// PartialEq enables types to be compared for equality
#[derive(PartialEq)]
/// the newtype pattern consists of wrapping a core type 
/// within a single-field struct or perhaps a tuple
/// Hostname is our newtype
struct Hostname(String);

/// creating a type alias
// type FileType = String;

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
// read function returns the "number of bytes read"
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    // make a copy of the data here
    // as save_to.append() will shrink the input Vec<T>
    let mut tmp = f.data.clone();
    // ensure that there is sufficient space to fit the incoming data
    // and minimizes allocations when data is inserted byte-by-byte
    let read_length = tmp.len();
    save_to.reserve(read_length);
    // allocate sufficient data in the save_to buffer to hold the contents of f
    save_to.append(&mut tmp);
    read_length
}

#[allow(unused_variables)]
fn main() {
    // File "inherits" all of String’s methods
    // let mut f1 = FileType::from("f1.txt");
    // open(&mut f1);
    // read(&mut f1, vec![]);
    // close(&mut f1);

    // creating a file from the File struct
    // creating a first instance of File
    let mut f2 = File {
        // String::from allows owned strings to be generated from string literals
        name: String::from("f2.txt"),
        // using the vec! macro to simulate an empty file
        data: vec![114, 117, 115, 116, 33],
    };
    
    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    // convert Vec<u8> to String
    // any bytes that are not valid UTF-8 are replaced with ?
    let text = String::from_utf8_lossy(&buffer);

    // accessing fields uses the dot operator
    let f2_name = &f2.name;
    let f2_length = &f2.data.len();

    println!("{:?}", f2);
    println!("{} is {} bytes long", f2_name, f2_length);
    // view the bytes 114, 117, 115, 116 & 33 as an actual word
    println!("{}", text);

    // creating an ordinary string to compare it withn the Hostname newtype
    let ordinary_string = String::from("localhost");
    let host = Hostname ( ordinary_string.clone() );
    // the following lines won’t compile because the compiler understands 
    // that Hostname and String are distinct
    /* 
    if host == ordinary_string {
        println!("huh?");
    };
    */
}
