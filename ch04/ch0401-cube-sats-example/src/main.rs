#![allow(unused_variables)]
/// coding along with Rust in Action by Tim McNamara
/// Chapter 4, Lifetimes, Ownership and Borrowing, chapter 4.1, 
/// 4.1 “Implementing” a Mock CubeSat Ground Station
/// 
/// code examples and comments are taken from the book
/// 

// enum to check the status of each satellite 
#[derive(Debug)]
enum StatusMessage {
    Ok,
}

// creating a type to model our satellites
// modelling a CubeSat as its own type
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

// using the CubeSat type within check_status()
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