use std::io::{self, Read};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read from stdin");

    let (rules, updates) = parse_input(&input);
    let mut sum_middle_pages = 0;

    for update in &updates {
        if is_correct_order(update, &rules) {
            let middle_page = find_middle_page(update);
            sum_middle_pages += middle_page;
        }
    }

    println!("Sum of middle pages of correctly ordered updates: {}", sum_middle_pages);

    let mut sum_corrected_middle_pages = 0;

    for update in updates {
        if !is_correct_order(&update, &rules) {
            let corrected_update = correct_order(update, &rules);
            let middle_page = find_middle_page(&corrected_update);
            sum_corrected_middle_pages += middle_page;
        }
    }

    println!("Sum of middle pages of corrected updates: {}", sum_corrected_middle_pages);
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut lines = input.trim().lines();
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut parsing_rules = true;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 2 {
                panic!("Invalid rule format: {}", line);
            }
            let x = parts[0].trim().parse().expect("Failed to parse rule part");
            let y = parts[1].trim().parse().expect("Failed to parse rule part");
            rules.push((x, y));
        } else {
            let update: Vec<i32> = line.split(',')
                .map(|s| s.trim().parse().expect("Failed to parse update part"))
                .collect();
            updates.push(update);
        }
    }

    (rules, updates)
}

fn is_correct_order(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let mut positions = HashMap::new();
    for (i, &page) in update.iter().enumerate() {
        positions.insert(page, i);
    }

    for &(x, y) in rules {
        if let (Some(&pos_x), Some(&pos_y)) = (positions.get(&x), positions.get(&y)) {
            if pos_x > pos_y {
                return false;
            }
        }
    }

    true
}

fn correct_order(mut update: Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut graph = HashMap::new();
    let mut in_degree = HashMap::new();

    for &page in &update {
        graph.entry(page).or_insert_with(HashSet::new);
        in_degree.entry(page).or_insert(0);
    }

    for &(x, y) in rules {
        if graph.contains_key(&x) && graph.contains_key(&y) {
            graph.get_mut(&x).unwrap().insert(y);
            *in_degree.get_mut(&y).unwrap() += 1;
        }
    }

    let mut queue = VecDeque::new();
    for (&page, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(page);
        }
    }

    let mut sorted = Vec::new();
    while let Some(page) = queue.pop_front() {
        sorted.push(page);
        if let Some(neighbors) = graph.get(&page) {
            for &neighbor in neighbors {
                let degree = in_degree.get_mut(&neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    sorted
}

fn find_middle_page(update: &Vec<i32>) -> i32 {
    let mid_index = update.len() / 2;
    update[mid_index]
}