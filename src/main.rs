use std::io;
use std::process::exit;

fn main() {
    println!("Enter expression: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read from stdin");

    let symbols = "/*+-";
    let trimmed = user_input.trim();

    let mut stack = Vec::new();
    let mut small_stack = String::new();
    for i in trimmed.chars() {
        if symbols.contains(i) {
            stack.push(small_stack);
            stack.push(i.to_string());
            small_stack = String::new();
        } else {
            small_stack.push(i);
        }
    }
    if !small_stack.is_empty() {
        stack.push(small_stack);
    }

    let mut stack_size = stack.len() - 1;
    for operation in symbols.chars() {
        let mut i = 0;
        while i <= stack_size {
            if stack[i] == operation.to_string() {
                match perform_operation(&stack[i - 1], &stack[i + 1], operation) {
                    Ok(result) => {
                        stack[i - 1] = result.to_string();
                        stack.remove(i);
                        stack.remove(i);
                        stack_size -= 2;
                        i -= 1;
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                        exit(1);
                    }
                }
            }
            i += 1;
        }
    }

    println!("The calculated value is: {}", stack[0]);
}

fn perform_operation(num1: &str, num2: &str, operator: char) -> Result<f32, String> {
    let left_num: f32 = num1.parse().map_err(|_| "Invalid number".to_string())?;
    let right_num: f32 = num2.parse().map_err(|_| "Invalid number".to_string())?;

    match operator {
        '+' => Ok(left_num + right_num),
        '-' => Ok(left_num - right_num),
        '*' => Ok(left_num * right_num),
        '/' => {
            if right_num == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(left_num / right_num)
            }
        }
        _ => Err(format!("Unknown operator: {}", operator)),
    }
}
