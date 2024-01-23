/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, chapter 2.10/11, arrays and slices
/// 
/// code examples and comments are taken from the book
/// 

fn main() {
    // 2 ways of creating arrays:
    // (1) creating an array as a comma-delimited list within square brackets
    // [1, 2, 3] denotes an array literal and Rust infers its type itself
    let one = [1, 2, 3];
    // [u8; 3] explicitly declares type of the array: 3 elements of u8
    let two: [u8; 3] = [1, 2, 3];
    // (2) from a repeat expression where you provide two values delimited by a semi-colon
    // repeat expression ([0; 3]) that expects a constant (0) to be repeated n times (3)
    let blank = [0; 3];
    // repeat expression with type signature
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank, blank2];
    
    // taking a reference to an array returns a slice
    // slices support iteration through arrays without needing to call iter()
    for a in &arrays {
        print!("{:?}; ", a);
        // arrays also have methods for iteration and manipulation
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n+10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t(Σ{:?} = {})", a, sum);
    }

    // simple slice example from https://doc.rust-lang.org/book/ch04-03-slices.html
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}
