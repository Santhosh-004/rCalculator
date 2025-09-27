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

    let mut it = 0;
    let mut stack_size = stack.len()-1;
    loop {
        let operation = symbols.chars().nth(it).unwrap().to_string();
        let mut i = 0;
        while i <= stack_size {
            let mut operation_performed = false;
            if stack[i] == operation {
                if operation == "/" {
                    let number1: i32 = stack[i-1].parse().unwrap();
                    let number2: i32 = stack[i+1].parse().unwrap();
                    if number2 == 0 {
                        println!("division by 0 happened, exiting");
                        exit(1);
                    }
                    let temp = number1/number2;
                    stack[i-1] = temp.to_string();
                    operation_performed = true;
                
                } else if operation == "*" {
                    let number1: i32 = stack[i-1].parse().unwrap();
                    let number2: i32 = stack[i+1].parse().unwrap();
                    let temp = number1*number2;
                    stack[i-1] = temp.to_string();
                    operation_performed = true;
                
                } else if operation == "+" {
                    let number1: i32 = stack[i-1].parse().unwrap();
                    let number2: i32 = stack[i+1].parse().unwrap();
                    let temp = number1+number2;
                    stack[i-1] = temp.to_string();
                    operation_performed = true;
                
                } else if operation == "-" {
                    let number1: i32 = stack[i-1].parse().unwrap();
                    let number2: i32 = stack[i+1].parse().unwrap();
                    let temp = number1-number2;
                    stack[i-1] = temp.to_string();
                    operation_performed = true;
                }
                if operation_performed {
                    stack.remove(i);
                    stack.remove(i);
                    stack_size -= 2;
                    i -= 1;
                }
            }
            i += 1;
        }
        it+=1;
        if it == 4 {
            break;
        }
    }

    println!("The calculated value is: {}", stack[0]);
    
}