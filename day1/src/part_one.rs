use std::path::Path;

pub fn run_puzzle<P>(filename: P) where P: AsRef<Path> {
    let mut nums: Vec<i32> = Vec::new();
    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines {
            if let Ok(input) = line {
                nums.push(run_input_line(input))
            }
        }
    }
    let mut result: i32 = 0;
    for num in nums.iter() {
        result += num;
    }
    println!("{}", result);
}

fn run_input_line(input: String) -> i32 {
    let mut digits: [char; 2] = ['x', 'x'];
    let mut current_digit = 0;
    for c in input.chars() {
        if c.is_numeric() {
            digits[current_digit] = c;
            current_digit = 1;
        }
    }
    let num_str: String;
    if digits[1] == 'x' {
        num_str = format!("{}{}", digits[0], digits[0]);
    } else {
        num_str = format!("{}{}", digits[0], digits[1]);
    }
    return num_str.parse::<i32>().unwrap();
}