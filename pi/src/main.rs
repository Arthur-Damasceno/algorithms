use std::io::{stdin, stdout, Write};

fn read_operations() -> u32 {
    let mut x = String::new();

    print!("How many operations?: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut x)
        .expect("Error while reading number");

    x.trim().parse().expect("Invalid number")
}

fn main() {
    let operations = read_operations();
    let mut pi = 0.;

    for i in 0..operations {
        let x = if i % 2 == 0 {
            4. / (1. + 2. * (i as f64))
        } else {
            -4. / (1. + 2. * (i as f64))
        };

        pi += x;
    }

    println!("The approximate value of pi is {pi}")
}
