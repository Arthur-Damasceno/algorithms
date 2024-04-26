use std::io::{stdin, stdout, Write};

const CYCLES: u32 = 100_000;

fn ln(num: f64) -> f64 {
    if num <= 0. {
        panic!("Number cannot be negative or null");
    }

    let mut sum = 0.;
    let interval = (num - 1.) / CYCLES as f64;

    for i in 0..CYCLES {
        sum += 1. / (1. + interval * (i as f64 + 0.5));
    }

    sum *= interval;

    sum
}

fn main() {
    let mut num = String::new();

    print!("Type a value: ");
    stdout().flush().unwrap();

    stdin().read_line(&mut num).expect("Cannot read the number");

    let num = num.trim().parse().expect("Invalid number");

    println!("ln({num}) = {}", ln(num));
}
