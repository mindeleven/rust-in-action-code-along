/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, Iteration, chapter 2.5, loop, 
/// loop -> the basis for Rust's looping constructs
/// the loop keyword provides more control than for or while
/// 
/// code examples and comments are taken from the book
/// 
/// https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
/// Rust provides a loop keyword to indicate an infinite loop
/// -> break statement can be used to exit a loop at anytime,
/// -> continue statement can be used to skip the rest of the iteration and start a new one

fn main() {
    // example from https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
    let mut count = 0u32;

    println!("let's count until infinity");

    loop {
        count += 1;

        println!("we're in an infinite loop and the count now is {}", count);

        if count == 3 {
            println!("three!");
            println!("skipped!");
            // skip the rest of this iteration
            continue;
        }

        println!("still looping if not skipped");

        if count == 10 {
            println!("infinity stops right here...................");
            break;
        }

    }

    // break from nested loops with loop labels
    // a loop label is an identifier prefixed with an apostrophe (')
    println!("\nexample for breaking from nested loops with loop labels:");
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                println!("x + y + z = {}", x + y + z);
                if x + y + z > 10 {
                    print!("time to break up\n");
                    break 'outer;
                }
            }
        }
    }

    // conditional testing
    // conditional tests return values
    // Rust is an expression based language which means all expressions return values
    // this allows to assign variables directly from the return value of a conditional expression
    println!("\nexample for assigning variables from the return value of a conditional expression:");
    let num_to_test = 1234561;
    let description = if is_even(num_to_test) {
        "even"
    } else {
        "odd"
    };
    println!("{} is {}", num_to_test, description);
}

/// example of helper function for conditional testing
fn is_even(a: i32) -> bool {
    if a % 2 == 0 {
        true
    } else {
        false
    }
}
