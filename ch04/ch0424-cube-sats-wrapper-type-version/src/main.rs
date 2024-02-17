/// coding along with Rust in Action by Tim McNamara
/// Chapter 4, Lifetimes, Ownership and Borrowing, 
/// chapter 4.5 Resolving Ownership Issues, 
/// chapter 4.5.4, Wrao data within speciality types
/// 
/// code examples and comments are taken from the book
/// 
/// a final strategy: using "wrapper" types that present a façade 
/// to the outside world of move semantics
/// but are doing something special under the hood
/// 
/// introducing a new notation: Rc<T>
/// Rc<T> means a "reference counted type T"
/// we could wrap a single instance of GroundStation in a Rc
/// providing shared access to each of the satellites
/// Rc<T> implements Clone
/// Rc<T> does not allow mutation
/// to allow mutation we need to wrap our wrapper
/// => Rc<RefCell<T>> is a type that can be used to perform interior mutability

use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug)]
struct GroundStation { 
    radio_frequency: f64
}

fn main() {
    // wrapping => enclosing the GroundStation instance in a call to Rc::new()
    let base: Rc<RefCell<GroundStation>> = Rc::new(
        RefCell::new(
            GroundStation { radio_frequency: 87.65 }
        )
    );

    println!("{:?}", base);
    println!("-----------------------------------------------------");

    // now let's open a new scope
    {
        let mut base_2 = base.borrow_mut();
        println!("{:?}", base_2);
        base_2.radio_frequency -= 12.34;
        println!("{:?}", base_2);
    }

    println!("-----------------------------------------------------");
    println!("{:?}", base);
    println!("-----------------------------------------------------");

    let mut base_3 = base.borrow_mut();
    println!("{:?}", base_3);
    base_3.radio_frequency += 43.21;
    println!("{:?}", base_3);

    println!("-----------------------------------------------------");
    // this will returned "borrowed"
    // value "borrowed" indicates that base has been mutably borrowed by somewhere else
    // and is no longer generally accessible
    println!("{:?}", base);
    println!("-----------------------------------------------------");
    
}
