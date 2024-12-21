use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
    Variable(String),
    Assign,
}

fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    for c in expression.chars() {
        match c {
            '0'..='9' | '.' => {
                buffer.push(c);
            }
            'a'..='z' | 'A'..='Z' => {
                buffer.push(c);
            }
            '+' | '-' | '*' | '/' => {
                if !buffer.is_empty() {
                    if let Ok(num) = buffer.parse() {
                        tokens.push(Token::Number(num));
                    } else {
                        tokens.push(Token::Variable(buffer.clone()));
                    }
                    buffer.clear();
                }
                tokens.push(Token::Operator(c));
            }
            '(' => {
                tokens.push(Token::LeftParen);
            }
            ')' => {
                if !buffer.is_empty() {
                    if let Ok(num) = buffer.parse() {
                        tokens.push(Token::Number(num));
                    } else {
                        tokens.push(Token::Variable(buffer.clone()));
                    }
                    buffer.clear();
                }
                tokens.push(Token::RightParen);
            }
            '=' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Variable(buffer.clone()));
                    buffer.clear();
                }
                tokens.push(Token::Assign);
            }
            ' ' => {
                if !buffer.is_empty() {
                    if let Ok(num) = buffer.parse() {
                        tokens.push(Token::Number(num));
                    } else {
                        tokens.push(Token::Variable(buffer.clone()));
                    }
                    buffer.clear();
                }
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    if !buffer.is_empty() {
        if let Ok(num) = buffer.parse() {
            tokens.push(Token::Number(num));
        } else {
            tokens.push(Token::Variable(buffer));
        }
    }

    tokens
}

fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    let precedence: HashMap<char, usize> = HashMap::from([
        ('+', 1),
        ('-', 1),
        ('*', 2),
        ('/', 2),
    ]);

    for token in tokens {
        match token {
            Token::Number(_) | Token::Variable(_) => output.push(token),
            Token::Operator(op) => {
                while let Some(top_op) = operators.last() {
                    if let Token::Operator(top_op_char) = top_op {
                        if precedence[&op] <= precedence[top_op_char] {
                            output.push(operators.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                operators.push(Token::Operator(op));
            }
            Token::LeftParen => operators.push(token),
            Token::RightParen => {
                while let Some(top_op) = operators.pop() {
                    if let Token::LeftParen = top_op {
                        break;
                    } else {
                        output.push(top_op);
                    }
                }
            }
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}

fn evaluate_rpn(tokens: Vec<Token>, variables: &HashMap<String, f64>) -> f64 {
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => stack.push(num),
            Token::Variable(var) => {
                if let Some(&val) = variables.get(&var) {
                    stack.push(val);
                } else {
                    panic!("Undefined variable: {}", var);
                }
            }
            Token::Operator(op) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => panic!("Unknown operator: {}", op),
                };
                stack.push(result);
            }
            _ => panic!("Unexpected token in RPN: {:?}", token),
        }
    }

    stack.pop().unwrap()
}

fn main() {
    let mut variables: HashMap<String, f64> = HashMap::new();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let tokens = tokenize(input);

        if tokens.contains(&Token::Assign) {
            if let (Token::Variable(var), rest) = (&tokens[0], &tokens[2..]) {
                let rpn = shunting_yard(rest.to_vec());
                let result = evaluate_rpn(rpn, &variables);
                variables.insert(var.clone(), result);
                println!("{}", result);
            }
        } else {
            let rpn = shunting_yard(tokens);
            let result = evaluate_rpn(rpn, &variables);
            println!("{}", result);
        }
    }
}
