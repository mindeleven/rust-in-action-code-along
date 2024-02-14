/// coding along with Rust in Action by Tim McNamara
/// Chapter 4, Lifetimes, Ownership and Borrowing, 
/// chapter 4.5 Resolving Ownership Issues, 
/// chapter 4.5.3, Duplicate the Value
/// 
/// code examples and comments are taken from the book
/// 

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a);

    println!("a: {:?}", a_status);

    let a_status = check_status(sat_a);
    
    println!("a: {:?}", a_status);
}