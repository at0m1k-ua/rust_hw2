use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    loop {
        print!("Введіть вираз у польській нотації: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        let tokens: Vec<&str> = input.split_whitespace().collect();

        match calculate_rpn(&tokens) {
            Ok(result) => println!("Результат: {}", result),
            Err(e) => println!("Помилка: {}", e),
        }
    }
}

fn calculate_rpn(tokens: &[&str]) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match *token {
            "+" | "-" | "*" | "/" => {
                if stack.len() < 2 {
                    return Err("Недостаточно операндов".to_string());
                }

                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let result = match *token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0.0 {
                            return Err("Ділення на нуль".to_string());
                        }
                        a / b
                    }
                    _ => unreachable!(),
                };

                stack.push(result);
            }
            _ => {
                match f64::from_str(*token) {
                    Ok(num) => stack.push(num),
                    Err(_) => return Err(format!("Невірний токен: {}", token)),
                }
            }
        }
    }

    if stack.len() != 1 {
        return Err("Невірна кількість операндів та операторів".to_string());
    }

    Ok(stack.pop().unwrap())
}
