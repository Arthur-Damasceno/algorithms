use std::io::{stdin, stdout, Write};

fn read_grade() -> f64 {
    let mut number = String::new();

    print!("What's the grade?: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut number)
        .expect("Error while reading number");

    let number: f64 = number.trim().parse().expect("Cannot parse number");

    number
}

fn main() {
    let first_grade = read_grade();
    let second_grade = read_grade();

    let average = (first_grade + second_grade) / 2.;

    if average >= 5. {
        println!("Approved!");
    } else {
        println!("Failed!");
    }
}
