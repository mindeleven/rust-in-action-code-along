#![allow(unused_variables)]
#![allow(dead_code)]
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
    // fn recv(&mut self) -> Option<Message> {
    //     self.mailbox.messages.pop()
    // }
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

/// defining new struct that contains a vector of Messages within its messages field
/// String has been aliased to Message below giving us the functionality of the String type
#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    // Mailbox.post() requires mutable access to itself and ownership over a Message
    fn post(&mut self, to: &CubeSat, msg: Message) {
        self.messages.push(msg);
    }
    
    // Mailbox.deliver() requires a shared reference to a CubeSat to pull out its id field
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                // actually an anti-pattern:
                // mutating a collection while it is being iterated over
                // legal here because of the return on the next line
                let msg = self.messages.remove(i);
                // if we find a message
                // return early with the Message wrapped in Some
                return Some(msg);
            }
        }
        // when no messages are found return None
        None
    }
}

/// type Message = String;
/// storing the Message in a struct so that it can live 
/// somewhere outside of the CubeSat instances
/// otherwise it would die when the for loop to create the CubeSats ends
#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

/// defining a struct to represent the ground station
struct GroundStation;

impl GroundStation {
    /*
    // &self indicates that GroundStation.send() only requires a read-only reference
    // recipient "to" is taking a mutable borrow (&mut) of the CubeSat instance
    // msg takes full ownership of its Message instance
    fn send(&self, to: &mut CubeSat, msg: Message) {
        // ownership of the Message instance transfers from msg 
        // into messages.push() as a local variable
        to.mailbox.messages.push(msg);
    }
    */
    // sending messages becomes a call to Mailbox.post()
    // yielding ownership of a Message
    fn send(&self, mailbox: &mut Mailbox, to: &CubeSat, msg: Message) {
        mailbox.post(to, msg);
    }

    // method that allows us to create a CubeSat instance on demand once
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
            mailbox: Mailbox { messages: vec![] }
        }
    }
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };
    
    let base = GroundStation {};
    
    // fetching the stored sat ids
    let sat_ids = fetch_sat_ids();
    
    for sat_id in sat_ids {
        // creating sat objects for the lifetime of the for loop
        let mut sat = base.connect(sat_id);
        base.send(
            &mut mail,
            &mut sat, 
            Message { to: sat_id, content: String::from("Hello from for loop no. 1")}
        );
    }

    // fetching the stored sat ids a second time
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);

        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }

}