use regex::Regex;
use std::io;

const OPERATOR_TYPES: [char; 4] = ['/', '*', '+', '-'];

fn get_highest_precedence(operators: &Vec<char>) -> char {
    let mut highest = operators[0];
    let op_pos = OPERATOR_TYPES
        .iter()
        .position(|&x| x == operators[0])
        .expect("invalid operator");

    for op in operators {
        let pos = OPERATOR_TYPES
            .iter()
            .position(|&x| x == *op)
            .expect("invalid operator");
        if pos < op_pos {
            highest = *op;
        }
    }
    highest
}

fn operate(x: f64, y: f64, operator: char) -> f64 {
    match operator {
        '/' => x / y,
        '*' => x * y,
        '+' => x + y,
        '-' => x - y,
        _ => panic!("Invalid operator"),
    }
}
fn calculate(input: &String) -> Result<f64, &str> {
    //obtain and format input
    let mut result: f64 = 0.0;
    let mut nums: Vec<Vec<char>> = vec![vec![]];
    let mut operators: Vec<char> = Vec::new();
    let mut last_was_operator = false;
    for c in input.trim().replace(" ", "").chars() {
        if OPERATOR_TYPES.contains(&c) {
            if !last_was_operator {
                operators.push(c);
                last_was_operator = true;
            } else {
                if c == '-' || c == '+' {
                    nums.push(vec![c]);
                    last_was_operator = false;
                } else {
                    return Err("Invalid input");
                }
            }
        } else {
            let re = Regex::new(r"\d|\(\)").unwrap();
            if re.is_match(&c.to_string()) {
                if last_was_operator {
                    nums.push(vec![c]);
                } else {
                    let length = nums.len() - 1;
                    nums[length].push(c);
                }
                last_was_operator = false;
            } else {
                panic!("Invalid number");
            }
        }
    }
    let mut formatted_nums: Vec<f64> = Vec::new();
    for arr in nums {
        let mut num_string = String::new();
        for c in arr {
            num_string.push(c);
        }
        let num: f64 = num_string.parse().expect("Invalid number");
        formatted_nums.push(num)
    }

    while operators.len() > 0 {
        let op = get_highest_precedence(&operators);
        let op_pos = operators
            .iter()
            .position(|&x| x == op)
            .expect("invalid operator");
        result = operate(formatted_nums[op_pos], formatted_nums[op_pos + 1], op);

        println!(
            "{} {} {} = {}",
            formatted_nums[op_pos],
            op,
            formatted_nums[op_pos + 1],
            result
        );
        operators.remove(op_pos);
        formatted_nums[op_pos] = result;
        formatted_nums.remove(op_pos + 1);
    }

    Ok(result)
}

//Example calculation: 5*9/15*7+3-10
fn main() {
    println!("Enter your calculation:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    calculate(&input).expect("Invalid input");
}
