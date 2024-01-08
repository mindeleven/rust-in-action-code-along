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

    }

}
