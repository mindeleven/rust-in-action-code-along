#![allow(unused_variables)]
/// coding along with Rust in Action by Tim McNamara
/// Chapter 4, Lifetimes, Ownership and Borrowing, 
/// chapter 4.1, “Implementing” a Mock CubeSat Ground Station, 
/// chapter 4.5.2, Use fewer long-lived values
/// 
/// code examples and comments are taken from the book
/// 
/// modification of the code example at ch0401-cube-sats-example
/// that's based on the first part of the chapter

/// functionality to return CubeSat identifiers
/// function is assumed to be some kind of database that's responsible for communication
/// when communication with a satellite is needed we create a new object
/// so there is no requirement for us to maintain live objects 
/// for the whole of the program’s duration
fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

/// enum to check the status of each satellite 
/// only primitive types have copy semantics whereas all other types have move semantics
#[derive(Debug)]
enum StatusMessage {
    Ok,
}

/// creating a type to model our satellites
/// modelling a CubeSat as its own type
#[derive(Debug)]
#[allow(dead_code)]
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
    let base = GroundStation {};


    // model with three CubeSats
    // ownership originates at the creation of the CubeSat object
    let mut sat_a = CubeSat {id: 0, mailbox: Mailbox { messages: vec![] }};

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("Hello there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    
    println!("t2: {:?}", sat_a);
    
    println!("msg: {:?}", msg);

    // now the return value of check_status() is the original CubeSat
    // the new let binding is "reset"
    // let sat_a = check_status(sat_a);

    // "waiting" ...
    // we can do it again without any compiler complains
    let sat_a = check_status(sat_a);

}