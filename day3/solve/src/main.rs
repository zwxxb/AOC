use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mul_re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    let mut part1_sum: i64 = 0;
    for caps in mul_re.captures_iter(&input) {
        let x: i64 = caps[1].parse().unwrap();
        let y: i64 = caps[2].parse().unwrap();
        part1_sum += x * y;
    }
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let mut part2_sum: i64 = 0;
    let mut mul_enabled = true;

    let mut pos = 0;
    let bytes = input.as_bytes();
    while pos < bytes.len() {
        if pos + 4 <= bytes.len() && &input[pos..pos+4] == "do()" {
            mul_enabled = true;
            pos += 4;
            continue;
        }
        if pos + 7 <= bytes.len() && &input[pos..pos+7] == "don't()" {
            mul_enabled = false;
            pos += 7;
            continue;
        }

        if let Some(mat) = mul_re.find(&input[pos..]) {
            if mat.start() == 0 {
                if mul_enabled {
                    let caps = mul_re.captures(&input[pos..]).unwrap();
                    let x: i64 = caps[1].parse().unwrap();
                    let y: i64 = caps[2].parse().unwrap();
                    part2_sum += x * y;
                }
                pos += mat.end();
                continue;
            }
        }
        pos += 1;
    }

    println!("Part 1 Result: {}", part1_sum);
    println!("Part 2 Result: {}", part2_sum);
}