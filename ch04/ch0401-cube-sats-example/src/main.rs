#![allow(unused_variables)]
/// coding along with Rust in Action by Tim McNamara
/// Chapter 4, Lifetimes, Ownership and Borrowing, chapter 4.1, 
/// 4.1 “Implementing” a Mock CubeSat Ground Station
/// 
/// code examples and comments are taken from the book
/// 
/// main focus of the chapter is on the borrow checker
/// -> the borrow checker checks that all access to data is legal
/// 
/// borrow checking relies on three inter-related concepts 
/// -> (1) ownership => every value in Rust is owned
///        ownership relates to cleaning values up when they’re no longer needed
///        owners cannot prevent other parts of the program from accessing their values
/// -> (2) lifetime => a value’s lifetime is the period when accessing that value is valid behavior
/// -> (3) borrowing => to borrow a value means to access it
///        there is no obligation to return the value back to its owner
///        it’s possible for many parts of the program to share access a value that has a single owner
/// 
/// the learning example for the chapter is a CubeSat constellation
/// -> CubeSats are miniature artificial satellites
/// -> a ground station is an intermediary between the operators and the satellites
/// -> a constellation is the collective noun for satellites in orbit
/// 

/// enum to check the status of each satellite 
/// only primitive types have copy semantics whereas all other types have move semantics
#[derive(Debug)]
enum StatusMessage {
    Ok,
}

/// creating a type to model our satellites
/// modelling a CubeSat as its own type
#[derive(Debug)]
struct CubeSat {
    id: u64,
    // adding Mailbox as a new field
    mailbox: Mailbox
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

/// defining new struct that contains a vector of Messages within its messages field
/// String has been aliased to Message below giving us the functionality of the String type
#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

/// defining a struct to represent the ground station
struct GroundStation;

impl GroundStation {
    // &self indicates that GroundStation.send() only requires a read-only reference
    // recipient "to" is taking a mutable borrow (&mut) of the CubeSat instance
    // msg takes full ownership of its Message instance
    fn send(&self, to: &mut CubeSat, msg: Message) {
        // ownership of the Message instance transfers from msg 
        // into messages.push() as a local variable
        to.mailbox.messages.push(msg);
    }
}

/// using the CubeSat type within check_status()
/// a 1st adjustment to check_status()
/// -> allows to give back the ownership of the CubeSats to the original variables
/// -> printout becomes a side effect
fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn main() {
    // model with three CubeSats
    // ownership originates at the creation of the CubeSat object
    let sat_a = CubeSat {id: 0, mailbox: Mailbox { messages: vec![] }};
    let sat_b = CubeSat {id: 1, mailbox: Mailbox { messages: vec![] }};
    let sat_c = CubeSat {id: 2, mailbox: Mailbox { messages: vec![] }};
    
    // now the return value of check_status() is the original CubeSat
    // the new let binding is "reset"
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
    // printout now is a side effect of the function and not needed here
    // println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // "waiting" ...
    // we can do it again without any compiler complains
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
    // println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

}