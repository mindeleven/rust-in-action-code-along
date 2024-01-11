/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, example 8, 
/// calculating values with complex numbers
/// 
/// demonstrates accessing a third party crate (num)
/// and two forms of initializing non-primitive data types
/// 
/// code examples and comments are taken from the book
/// 
use num::complex::Complex;

fn main() {
    // initializing non-primitive data types by literal syntax
    // types do not require constructor methods to be created
    // they can be initialized by using the type name and assigning their fields
    let a = Complex { re: 2.1, im: -1.2 };
    // initializing non-primitive data types by the new static method
    // many types implement a new method for simplicity
    // the Complex type has two fields:
    // 1 - re for representing the real part
    // 2 - im for representing the imaginary part
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
