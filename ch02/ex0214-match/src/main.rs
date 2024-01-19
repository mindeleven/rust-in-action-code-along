/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, Iteration, chapter 2.5, match, 
/// match -> a safer alternative to if/else blocks
/// -> will warn you if you haven't considered a relevant alternative
/// -> can be used like a C switch
/// match returns immediately when a match is found
/// 
/// code examples and comments are taken from the book
/// 

fn main() {
    // example from https://doc.rust-lang.org/rust-by-example/flow_control/match.html
    let number = 12;

    println!("What can you tell me about {}?", number);

    match number {
        // match a single value
        1 => println!("One"),
        // match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // the ..= syntax matches an inclusive range
        13..=19 => println!("A teen"),
        // catch-all arm: handle the rest
        _ => println!("It's a number like any other number")
    }

    // match as an expression
    let boolean = true;

    let binary = match boolean {
        // the arms of a match must cover all the possible values
        // which isn't too hard with a boolean
        true => 1,
        false => 0,
    };

    println!("{} => {}", boolean, binary);

    // needle in a haystack example with match
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 1430 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!(">>>> congrats, haystack item {} is a hit!", item)
        } else {
            println!("haystack item {} is a miss you loser", item)
        }
    }

}
