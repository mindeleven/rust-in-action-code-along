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
/// -> (1) ownership => ownership relates to cleaning values up when they’re no longer needed
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
#[derive(Debug)]
enum StatusMessage {
    Ok,
}

/// creating a type to model our satellites
/// modelling a CubeSat as its own type
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

/// using the CubeSat type within check_status()
fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    // model with three CubeSats
    let sat_a = CubeSat {id: 0};
    let sat_b = CubeSat {id: 1};
    let sat_c = CubeSat {id: 2};

    let a_status: StatusMessage = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // "waiting" ...
    let a_status: StatusMessage = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

}