/// coding along with Rust in Action by Tim McNamara
/// Chapter 4, Lifetimes, Ownership and Borrowing, 
/// chapter 4.5 Resolving Ownership Issues, 
/// chapter 4.5.3, Duplicate the Value
/// 
/// code examples and comments are taken from the book
/// 
/// an alternative to Having a single owner is to simply copy values
/// primitive types, such as integers, are a good example of that
///
/// types can opt into two modes of being copied: Clone and Copy
/// => Copy acts implicitly whenever ownership would otherwise be moved
///    the bits of object a are replicated to create object b
/// => Clone acts explicitly
///    types that implement Clone have a .clone() method that is permitted 
///    to do whatever it needs to create a new type
/// 
///    to implement Copy, your types must consist of types that themselves implement Copy
/// 

/// struct consists of types that contain types that themselves implement Copy
/// so implementing copy for the type isn't a problem
/// adding #[derive(Copy)] tells the compiler to add an implementation itself
#[derive(Copy, Clone, Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

/// implementing the Copy trait manually for status message
impl Copy for StatusMessage { }

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
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