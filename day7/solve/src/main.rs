use itertools::Itertools;
use std::io::{self, Read};

fn evaluate_expression(numbers: &[i64], operators: &[&str]) -> i64 {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            "+" => result += numbers[i + 1],
            "*" => result *= numbers[i + 1],
            "||" => {
                let concat = concatenate(result, numbers[i + 1]);
                result = concat;
            }
            _ => {}
        }
    }
    result
}

fn concatenate(a: i64, b: i64) -> i64 {
    let mut multiplier = 1;
    let mut temp = b;
    while temp > 0 {
        multiplier *= 10;
        temp /= 10;
    }
    a * multiplier + b
}

fn can_be_true(test_value: i64, numbers: &[i64], operators: &[&str]) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == test_value;
    }

    let num_ops = numbers.len() - 1;

    let operator_sets: Vec<&[&str]> = std::iter::repeat(&operators[..]).take(num_ops).collect();

    for ops in operator_sets.iter().map(|&s| s.iter()).multi_cartesian_product() {
        let op_slice: Vec<&str> = ops.into_iter().map(|s| *s).collect();
        if evaluate_expression(numbers, &op_slice) == test_value {
            return true;
        }
    }

    false
}

fn main() {
    
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

   
    let part1_operators = ["+", "*"];
    let part1_total: i64 = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
               
                return 0;
            }

            let test_value: i64 = match parts[0].parse() {
                Ok(val) => val,
                Err(_) => return 0,
            };

            let numbers: Vec<i64> = parts[1]
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();

            if can_be_true(test_value, &numbers, &part1_operators) {
                test_value
            } else {
                0
            }
        })
        .sum();

    println!("Part 1 Total Calibration Result: {}", part1_total);

    // Part 2: "+" , "*", and "||" operators
    let part2_operators = ["+", "*", "||"];
    let part2_total: i64 = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
               
                return 0;
            }

            let test_value: i64 = match parts[0].parse() {
                Ok(val) => val,
                Err(_) => return 0,
            };

            let numbers: Vec<i64> = parts[1]
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();

            if can_be_true(test_value, &numbers, &part2_operators) {
                test_value
            } else {
                0
            }
        })
        .sum();

    println!("Part 2 Total Calibration Result: {}", part2_total);
}