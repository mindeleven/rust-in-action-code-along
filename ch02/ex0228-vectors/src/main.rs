/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, chapter 2.10.3, vectors
/// 
/// code examples and comments are taken from the book
/// and from Rust documentation Storing Lists of Values with Vectors
/// @ https://doc.rust-lang.org/book/ch08-01-vectors.html
///
/// VECTORS
/// -> collection type, Vec<T>, aka a vector
/// -> allows you to store more than one value in a single data structure 
///    that puts all the values next to each other in memory
/// -> all values have to be of the same type

fn main() {
    // creating an empty vector to hold values of type i32
    // Vectors are implemented using generics
    let mut v: Vec<i32> = Vec::new();
    // adding elements to the vector with push
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // creating a new vector with the vec! macro
    // Rust will infer the type of value you want to store
    let v2 = vec![1, 2, 3, 4, 5];
    println!("contains of v2: {:?}", v2);

    // reading elements of vectors
    // like so often there are two ways to reference a value stored in a vector
    // (1) via indexing or (2) using the get method
    let third = &v2[2];
    println!("third element of vector v2: {}", third);
    
    // get returns an Option<&T> that we can use with match
    let fourth = v2.get(3);
    match fourth {
        Some(fourth) => println!("fourth element of vector v2: {}", fourth),
        None => println!("v2 doesn't have an element at this spot")
    }
    
    // the get method returns None without panicking 
    // if it's passed an index that is outside the vector
    let no_panic = v2.get(100);
    match no_panic {
        Some(no_panic) => println!("100th element of vector v2: {}", no_panic),
        None => println!("v2 doesn't have an element at spot 100")
    }

    // attempting to add an element to a vector while holding a reference to an item
    // will result in a compile error
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    /*
    let first = &v[0]; // immutable borrow occurs here
    v.push(6); // mutable borrow occurs here
    println!("The first element is: {first}"); // immutable borrow later used here
    */

    // iterating over the values in a vector
    let vec = vec![100, 567, 99, 12];
    // printing each element in a vector using a for loop
    for v in &vec {
        println!("{}", v);
    }

    // iterating over mutable references to each element in a mutable vector 
    // in order to make changes to all the elements
    let mut mut_vec = vec![100, 567, 99, 12, 33, 98];
    // iterating over mutable references to elements in a vector
    for v in &mut mut_vec {
        // to change the value a mutable reference refers to, use the * dereference operator
        *v += 50;
    }
    println!("mutated vector: {:?}", mut_vec);

}
