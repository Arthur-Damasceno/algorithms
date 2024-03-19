use std::io::{stdin, stdout, Write};

fn read_sides() -> [f64; 3] {
    let mut numbers = String::new();

    print!("Type three numbers: ");
    stdout().flush().unwrap();

    stdin().read_line(&mut numbers).expect("Cannot read sides");

    let mut iter = numbers.trim().split(' ');

    [
        parse_number!(iter),
        parse_number!(iter),
        parse_number!(iter),
    ]
}

#[macro_export]
macro_rules! parse_number {
    ($iter:expr) => {
        $iter
            .next()
            .expect("There's no number")
            .parse::<f64>()
            .expect("Error parsing number")
    };
}

fn main() {
    let sides = read_sides();

    if sides[0] <= 0. || sides[1] <= 0. || sides[2] <= 0. {
        return println!("The sides of the triangle cannot be null or negative!");
    }

    if sides[0] >= sides[1] + sides[2] || sides[0] <= (sides[1] - sides[2]).abs() {
        return println!("The numbers cannot be the sides of a triangle!");
    }

    if sides[0] == sides[1] && sides[0] == sides[2] {
        println!("It's a equilateral triangle.")
    } else if sides[0] == sides[1] || sides[0] == sides[2] || sides[1] == sides[2] {
        println!("It's a isoceles triangle.")
    } else {
        println!("It's a scalene triangle.")
    }
}
