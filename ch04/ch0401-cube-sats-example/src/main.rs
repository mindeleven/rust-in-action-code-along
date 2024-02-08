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

fn main() {
    // model with three CubeSats
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;

    let a_status = check_status(sat_a);

    print!("a: {:?}", a_status);

}

fn check_status(sat_id: u64) -> StatusMessage {
    StatusMessage::Ok
}