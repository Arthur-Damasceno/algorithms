use std::io::{stdin, stdout, Write};

fn main() {
    let mut number = String::new();

    print!("What's the number?: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut number)
        .expect("Error while reading number");

    let number: f64 = number.trim().parse().expect("Cannot parse number");
    let double = number * 2.;

    println!("The double of {number} is {double}");
}
