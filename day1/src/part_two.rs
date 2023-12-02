use std::path::Path;

trait DigitMachine {
    fn machine_input(&mut self, c: char) -> char;
    fn reset(&mut self);
}

struct NumericDigitMachine {}

impl NumericDigitMachine {
    pub fn new() -> Self {
        return Self {}
    }
}

impl DigitMachine for NumericDigitMachine {
    fn machine_input(&mut self, c: char) -> char {
        if c.is_numeric() {
            return c;
        }
        return 'x';
    }

    fn reset(&mut self) {}
}

struct StrDigitMachine {
    chars: Vec<char>,
    digit: char,
    end_index: usize,
    index: usize,
}

impl StrDigitMachine {
    pub fn new(s: &str, digit: char) -> Self {
        let chars: Vec<char> = s.chars().collect();
        let end = chars.len() - 1;
        return Self {
            chars: chars,
            digit: digit,
            end_index: end,
            index: 0,
        }
    }
}

impl DigitMachine for StrDigitMachine {
    fn machine_input(&mut self, c: char) -> char {
        // Check if the input is equal to the current indexed
        // character.
        if self.chars[self.index] != c {
            self.index = 0;
            // Edge case: The new character might still be the same as the
            // first character in the machine sequence.
            if self.chars[0] == c {
                self.index = 1;
            }
            return 'x';
        }

        // If we are at the final character the last comparison succeeded,
        // then we have found what this machine is looking for.
        if self.index == self.end_index {
            self.index = 0;
            return self.digit;
        }

        self.index += 1;
        return 'x';
    }

    fn reset(&mut self) {
        self.index = 0;
    }
}

pub fn run_puzzle<P>(filename: P) where P: AsRef<Path> {
    let mut machines: Vec<Box<dyn DigitMachine>> = vec![
        Box::new(NumericDigitMachine::new()),
        Box::new(StrDigitMachine::new("zero", '0')),
        Box::new(StrDigitMachine::new("one", '1')),
        Box::new(StrDigitMachine::new("two", '2')),
        Box::new(StrDigitMachine::new("three", '3')),
        Box::new(StrDigitMachine::new("four", '4')),
        Box::new(StrDigitMachine::new("five", '5')),
        Box::new(StrDigitMachine::new("six", '6')),
        Box::new(StrDigitMachine::new("seven", '7')),
        Box::new(StrDigitMachine::new("eight", '8')),
        Box::new(StrDigitMachine::new("nine", '9')),
    ];

    let mut nums: Vec<i32> = Vec::new();
    if let Ok(lines) = crate::read_lines(filename) {
        for line in lines {
            if let Ok(input) = line {
                nums.push(run_input_line(&mut machines, input));
                for machine in machines.iter_mut() {
                    machine.reset();
                }
            }
        }
    }
    let mut result: i32 = 0;
    for num in nums.iter() {
        result += num;
    }
    println!("{}", result);
}

fn run_input_line(machines: &mut Vec<Box<dyn DigitMachine>>, input: String) -> i32 {
    let mut digits: [char; 2] = ['x', 'x'];
    let mut curr_digit = 0;
    for c in input.chars() {
        let mut digit_result = 'x';
        for machine in machines.iter_mut() {
            let machine_result = machine.machine_input(c);
            if machine_result != 'x' {
                digit_result = machine_result;
            }
        }
        if digit_result != 'x' {
            digits[curr_digit] = digit_result;
            curr_digit = 1;
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