/// coding along with Rust in Action by Tim McNamara
/// Chapter 1, Introducing Rust, example 2, basic processing of some CSV data
/// code examples and comments are taken from the book
/// 
fn main() {
    // csv data example
    let penguin_data ="\
        common name,length (cm)
        Little penguin,33
        Yellow-eyed penguin,65
        Fiordland penguin,60
        Invalid,data
    ";

    // read the lines of the csv data
    let records = penguin_data.lines();
    
    // enumerate returns an iterator that yields the current count 
    // and the element during iteration
    // https://doc.rust-lang.org/std/iter/struct.Enumerate.html
    for (i, record) in records.enumerate() {

        // read the lines of the csv data but skip the header
        // and skip the lines with only whitespace
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        
        // collect the records into a vector
        // vectors are collections that can dynamically expand
        // the underscore (<_>) asks the compiler to infer the type of the elements
        let fields: Vec<_> = record
            // split record into substrings at the comma delimiter
            .split(',')
            // map makes use of a high order function
            // trims the whitespace from every field
            .map(|field| field.trim())
            .collect();
        
        // the following code block only makes sense if debugging is enabled
        // exclamation mark (!) indicates a macro invocation
        if cfg!(debug_assertions) {
            // eprintln! prints to standard error
            // {:?} syntax prints out default debugging representation
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        
        // get certain entry by indexing the collection with an integer
        let name = fields[0];
        
        // parsing the string into another types with parse
        // the type information has to be provided on the left-hand side
        // either returns a value or an error value wrapped in a Result
        // underscore requests the compiler to infer the error type itself
        let maybe_length: Result<f32, _> = fields[1].parse();
        
        // skip data in case of error
        if maybe_length.is_err() {
            continue;
        }
        
        // unwrap the value from the Result
        let length = maybe_length.unwrap();
        
        // println! prints to stdout
        // {} syntax indicates programmer-defined method to represent
        // the value as a string should be used
        println!("{}, {}cm", name, length);

        // cargo run returns output with debugging info because of cfg!(debug_assertions)
        // to get output w/o debugging information run
        // cargo run --release
        // output can be further reduced by adding the -q flag (-q for "quiet")
        // cargo run -q --release
    }

}
