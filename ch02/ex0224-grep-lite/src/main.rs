/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, chapter 2.9, creating grep-lite
/// 
/// code examples and comments are taken from the book
/// 
fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";
    // line counter no longer needed when enumerate() is used
    // let mut line_num: usize = 1;
    
    // the lines method returns an iterator over quote
    // the iterator can be chained with enumerate()
    for (i, line) in quote.lines().enumerate() {
        // searching for text using method syntax
        if line.contains(search_term) {
            println!("{}: {}", i+1, line);
        }
        // line_num += 1;
    }
}
