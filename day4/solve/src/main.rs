use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let grid: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let target = ['X', 'M', 'A', 'S'];
    let directions = [
        (-1, -1), 
        (-1, 0),  
        (-1, 1),  
        (0, -1),  
        (0, 1),   
        (1, -1),  
        (1, 0),  
        (1, 1),   
    ];

    let rows = grid.len();
    if rows == 0 {
        println!("0");
        return;
    }
    let cols = grid[0].len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if matches_direction(&grid, i, j, dx, dy, &target, rows, cols) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn matches_direction(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
    target: &[char],
    rows: usize,
    cols: usize,
) -> bool {
    let mut xi = x as i32;
    let mut yi = y as i32;

    for &ch in target {
        if xi < 0 || xi >= rows as i32 || yi < 0 || yi >= cols as i32 {
            return false;
        }
        if grid[xi as usize][yi as usize] != ch {
            return false;
        }
        xi += dx;
        yi += dy;
    }

    true
}