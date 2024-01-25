/// coding along with Rust in Action by Tim McNamara
/// Chapter 2, Language Foundations, chapter 2.10, 
/// creating grep-lite with vectors
/// 
/// code examples and comments are taken from the book
/// 
/// The task at hand is to expand the feature set of the grep-lite utility
/// -> we want the ability to store n lines of context around a match
/// -> to minimize code complexity, we’ll use a two pass strategy
/// -> (1) first pass, we’ll tag lines that match
/// -> (2) second pass, we’ll collect lines that are within n lines of each of the tags
/// 
/// bringing the Regex type from the regex crate into local scope
use regex::Regex;
use clap::{App, Arg};

fn main() {
    // incrementally build up a command argument parser
    // each argument takes an Arg
    // cargo run -- <pattern>
    // example: cargo run -- fever
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true)
        )
        .get_matches();
    
    let pattern = args.value_of("pattern").unwrap();
    // unwrap() "unwraps" a Result, crashing if an error occurs
    let re = Regex::new(pattern).unwrap();

    // number of context lines we want to store before and after
    let context_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    // let's see what we find with regex
    println!("let's see what we find with regex");
    for line in haystack.lines() {
        let contains_substring = re.find(line);
        // replacing contains() method with a match block
        match contains_substring {
            // Some(T) is the positive case of an Option
            // it means that re.find() has been successful
            Some(_) => println!("{}", line),
            // None is the negative case of an Option
            // () can be thought of as a null placeholder value here.
            None => ()
        }
    }

    // tag vector to hold line numbers where matches occur
    let mut tags: Vec<usize> = Vec::new();
    
    // ctx vector contains a vector per match to hold the match's context lines
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    // (1) first pass, we’ll tag lines that match
    for (i, line) in haystack.lines().enumerate() {
        // checking if line contains needle
        if line.contains(needle) {
            // store the line number that got a hit
            tags.push(i);
            // Vec::with_capacity(n) reserves space for n items
            // no explicit type signature is required here
            // as it can be inferred via the definition of ctx
            let v = Vec::with_capacity(2*context_lines + 1);
            ctx.push(v);
        }
    }

    println!("lines with hits: {:?}", tags);
    println!("context lines: {:?}", ctx);

    if tags.len() == 0 {
        // no matches?
        // exit early
        return;
    }
    
    // (2) second pass, we’ll collect lines that are within n lines of each of the tags
    // for each tag, at every line, check to see if we are nearby a match
    // if we are, add that line to the relevant Vec<T> within ctx
    for (i, line) in haystack.lines().enumerate() {

        for (j, tag) in tags.iter().enumerate() {
            // tag is of type usize
            // usize.saturating_sub() is subtraction that returns 0 on integer underflow 
            // rather than crashing the program
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                // copy line into a new String and store that locally for each match
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }

    }

    for local_ctx in ctx.iter() {
        println!("Needle found in:");
        // ref line informs the compiler that we wish to borrow this value, rather than move it
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
    
}
