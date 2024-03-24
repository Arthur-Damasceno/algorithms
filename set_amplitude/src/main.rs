use std::io::{stdin, stdout, Write};

fn main() {
    let mut numbers = Option::<(f64, f64)>::None;

    let mut number = String::new();

    loop {
        print!("Type a number: ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut number)
            .expect("Error while reading number");

        if let Ok(number) = number.trim().parse::<f64>() {
            if let Some((ref mut largest, ref mut smallest)) = numbers {
                if number > *largest {
                    *largest = number;
                } else if number < *smallest {
                    *smallest = number;
                }
            } else {
                numbers = Some((number, number));
            }
        } else {
            if let Some((largest, smallest)) = numbers {
                let amplitude = largest - smallest;
                println!("The amplitude is {amplitude}");
            } else {
                println!("There's no numbers");
            }

            break;
        }

        number.clear();
    }
}
