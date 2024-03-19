use std::io::{stdin, stdout, Write};

fn read_grade() -> f64 {
    let mut grade = String::new();

    print!("Type the grade: ");
    stdout().flush().unwrap();

    stdin().read_line(&mut grade).expect("Cannot read grade");

    let grade = grade.trim().parse().expect("Invalid number");

    if grade < 0. || grade > 10. {
        panic!("Invalid grade")
    }

    grade
}

fn main() {
    let (g1, g2) = (read_grade(), read_grade());

    let average = (2. * g1 + 3. * g2) / 5.;

    if average < 3. {
        println!("Failed!")
    } else if average < 7. {
        let grade = 10. - average;

        println!("You need {grade} in the final exam to pass!")
    } else {
        println!("Approved!")
    }
}
