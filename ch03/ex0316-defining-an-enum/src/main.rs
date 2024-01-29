/// coding along with Rust in Action by Tim McNamara
/// Chapter 3, Compound Data Types, chapter 3.5, Defining and making use of enum 
/// 
/// code examples and comments are taken from the book
/// 

/// enabling the enum to be printed to the screen
#[derive(Debug)]
enum Event {
    // creating three variants of Event
    // including one value for unrecognized events
    Update,
    Delete,
    Unknown
}

// defining a Message type as a convenient name for String for use in this crate’s context
type Message = String;

// function for parsing a line and converting it into semi-structured data
fn parse_log(line: &'static str) -> (Event, Message) {
    // line.splitn() returns an iterator
    // collect() consumes this iterator and returns Vec<T>
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    
    // line.splitn() is supposed to split log into two parts
    // return an error if this doesn't happen
    if parts.len() == 1 {
        return (Event::Unknown, String::from(line))
    }
    
    // assign each part to a variable for future use
    let event = parts[0];
    let rest = String::from(parts[1]);
    
    // now let's see what event we have
    match event {
        // return structured data if we match a known event
        "update" | "UPDATE" => (Event::Update, rest),
        "delete" | "DELETE" => (Event::Delete, rest),
        // return the whole line if we don’t recognize the event type
        _ => (Event::Unknown, String::from(line))
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
