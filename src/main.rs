use std::io::Read;

fn main() {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).expect("Error!");

    check(&input);
}

fn check(input: &str) {
    let mut parts = input.split_whitespace();

    let first_number: i128 = parts
        .next()
        .expect("Empty!")
        .parse()
        .expect("Not a Number!");

    let second_number: i128 = parts
        .next()
        .expect("Empty!")
        .parse()
        .expect("Not a Number!");

    let sum: i128 = first_number + second_number;

    println!("{}", sum);
}