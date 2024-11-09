use std::io;

fn get_number() -> f64 {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Failed to read line");
    let num: f64 = str.trim().parse().expect("Please enter a valid number");
    num
}

fn main() {
    loop {
        println!("Enter the first number:");
        let num1 = get_number();

        println!("Enter an operator (+, -, *, /):");
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("Failed to read line");
        let operator = operator.trim();

        println!("Enter the second number:");
        let num2 = get_number();

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero is not allowed.");
                    continue;
                } else {
                    num1 / num2
                }
            },
            _ => {
                println!("Invalid operator!");
                continue;
            }
        };

        println!("Result: {}", result);

        println!("Do you want to perform another calculation? (y/n):");
        let mut continue_input = String::new();
        io::stdin().read_line(&mut continue_input).expect("Failed to read line");
        if continue_input.trim().to_lowercase() != "y" {
            break;
        }
    }
}
