use std::io::{stdin, stdout, Write};

fn read_coeficients() -> (f64, f64, f64) {
    let mut numbers = String::new();

    print!("Type the coefficients of the function: ");
    stdout().flush().unwrap();

    stdin().read_line(&mut numbers).expect("Cannot read coefficients");

    let mut iter = numbers.trim().split(' ');

    (parse_number!(iter), parse_number!(iter), parse_number!(iter))
}

#[macro_export]
macro_rules! parse_number {
    ($iter:expr) => {
        $iter.next().expect("There's no number").parse::<f64>().expect("Error parsing number")
    };
}

fn main() {
    let (a, b, c) = read_coeficients();

    if a == 0. {
        return println!("Not a quadratic function")
    }

    let delta = b.powi(2) - 4. * a * c;

    if delta < 0. {
        return println!("There's no roots")
    } else if delta == 0. {
        let x = - b / (2. * a);
        return println!("The root is {x}")
    }

    let (x1, x2) = (
        (- b - delta.sqrt()) / (2. * a),
        (- b + delta.sqrt()) / (2. * a)
    );

    println!("The roots are {x1} and {x2}")
}
