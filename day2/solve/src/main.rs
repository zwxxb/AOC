use std::io::{self, BufRead};

fn check_safety(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;
    let mut valid_differences = true;

    for window in levels.windows(2) {
        let diff = window[1] - window[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            valid_differences = false;
            break;
        }

        if diff <= 0 {
            is_increasing = false;
        }
        if diff >= 0 {
            is_decreasing = false;
        }
    }

    (is_increasing || is_decreasing) && valid_differences
}

fn main() {
    let stdin = io::stdin();
    let mut safe_reports = 0;

    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            if l.trim().is_empty() || !l.chars().any(|c| c.is_digit(10)) {
                continue;
            }

            let levels: Vec<i32> = l
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            if levels.len() < 2 {
                continue;
            }
            if check_safety(&levels) {
                safe_reports += 1;
                continue;
            }
            let mut found_safe = false;
            for skip_idx in 0..levels.len() {
                let test_levels: Vec<i32> = levels.iter()
                    .enumerate()
                    .filter(|&(i, _)| i != skip_idx)
                    .map(|(_, &x)| x)
                    .collect();
                
                if check_safety(&test_levels) {
                    found_safe = true;
                    break;
                }
            }

            if found_safe {
                safe_reports += 1;
            }
        }
    }

    println!("Number of safe reports with Problem Dampener: {}", safe_reports);
}