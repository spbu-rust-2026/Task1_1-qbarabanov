use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    
    io::stdin().read_to_string(&mut input).expect("Error!");

    let mut parts = input.split_whitespace();

    let first_number: i128 = parts
        .next()
        .expect("Empty!")
        .parse()
        .expect("Not a number!");

    let second_number: i128 = parts
        .next()
        .expect("Empty!")
        .parse()
        .expect("Not a number!");

    println!("{}", first_number + second_number);
}