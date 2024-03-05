use std::io::{self, Write};

// Define an enum called Operation
#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    // Helper function to read input from the user
    fn read_input() -> f64 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().parse().expect("Invalid input")
    }

    // Prompt the user to input numbers and operation
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    let num1: f64 = read_input();

    print!("Enter the operation (+, -, *, /): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // trim spaces
    let operation_str = input.trim();

    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    let num2: f64 = read_input();

    // conver to enum
    let operation = match operation_str.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            println!("Error: Invalid operation!");
            return;
        }
    };

    // Perform the calculation
    let result = match operation {
        Operation::Add => num1 + num2,
        Operation::Subtract => num1 - num2,
        Operation::Multiply => num1 * num2,
        Operation::Divide => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error: Cannot divide by zero!");
                return;
            }
        }
    };

    // Print the result
    println!("Result: {}", result);
}
