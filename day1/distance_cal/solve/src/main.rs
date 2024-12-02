use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() >= 2 {
                if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    left.push(a);
                    right.push(b);
                }
            }
        }
    }

    if left.len() != right.len() {
        eprintln!("The two lists must have the same number of elements.");
        return;
    }

    // Part 1: Calculate Total Distance
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();
    left_sorted.sort();
    right_sorted.sort();

    let total_distance: i32 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Total distance: {}", total_distance);

    // Part 2: Calculate Similarity Score
    let mut frequency_map: HashMap<i32, usize> = HashMap::new();
    for &num in &right {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = left
        .iter()
        .map(|&num| {
            let count = frequency_map.get(&num).unwrap_or(&0);
            num * (*count as i32)
        })
        .sum();

    println!("Similarity score: {}", similarity_score);
}