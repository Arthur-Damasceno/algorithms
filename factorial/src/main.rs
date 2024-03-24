use std::io::stdin;

fn factorial(x: u16) -> u128 {
    if x <= 1 {
        return 1;
    }

    let mut x = x as u128;

    for i in 2..x {
        x *= i;
    }

    x
}

fn main() {
    let mut number = String::new();

    stdin()
        .read_line(&mut number)
        .expect("Error while reading number");

    let number = number.trim().parse().expect("Invalid number");

    let x = factorial(number);

    println!("The factorial of {number} is {x}");
}
