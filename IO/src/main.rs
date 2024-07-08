use std::io;

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    input_string.trim().parse().expect("Please type a number!")
}

fn main() {
    println!("Solve a quadratic equation");

    let a = read_input("Insert A:");
    let b = read_input("Insert B:");
    let c = read_input("Insert C:");

    let discriminant: f64 = (b * b) - 4.0 * (a * c);

    if discriminant > 0.0 {
        let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let x2 = (-b - discriminant.sqrt()) / (2.0 * a);

        println!(
            "Solved,\n there are 2 roots.\n D = {}\n Root 1 = {}\n and Root 2 = {}",
            discriminant, x1, x2
        );
    } else if discriminant == 0.0 {
        let x = -b / (2.0 * a);

        println!("Solved,\n there is 1 root.\n D = 0\n Root = {}", x);
    } else {
        println!("There are no roots \n D = {}", discriminant);
    }
}
