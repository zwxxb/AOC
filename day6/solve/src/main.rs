use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position { x: self.x, y: self.y.saturating_sub(1) },
            Direction::Right => Position { x: self.x + 1, y: self.y },
            Direction::Down => Position { x: self.x, y: self.y + 1 },
            Direction::Left => Position { x: self.x.saturating_sub(1), y: self.y },
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let map: Vec<Vec<char>> = stdin.lock().lines()
        .map(|line| line.expect("Failed to read line").chars().collect())
        .collect();

    let mut guard_pos = Position { x: 0, y: 0 };
    let mut guard_dir = Direction::Up;

    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                guard_pos = Position { x, y };
                guard_dir = Direction::Up;
            } else if cell == '>' {
                guard_pos = Position { x, y };
                guard_dir = Direction::Right;
            } else if cell == 'v' {
                guard_pos = Position { x, y };
                guard_dir = Direction::Down;
            } else if cell == '<' {
                guard_pos = Position { x, y };
                guard_dir = Direction::Left;
            }
        }
    }

    let mut visited_positions = HashSet::new();
    visited_positions.insert(guard_pos);

    let mut loop_counter = 0;
    let max_loops = 1000000; // Adjust as needed

    loop {
        loop_counter += 1;
        if loop_counter > max_loops {
            println!("Reached maximum iterations.");
            break;
        }

        let next_pos = guard_pos.step(guard_dir);
        if next_pos.y >= map.len() || next_pos.x >= map[0].len() || map[next_pos.y][next_pos.x] == '#' {
            guard_dir = guard_dir.turn_right();
            // Debug
            println!("Turned right. New direction: {:?}", guard_dir);
        } else {
            guard_pos = next_pos;
            visited_positions.insert(guard_pos);
            // Debug
            println!("Moved to: ({}, {:?})", guard_pos.x, guard_pos.y);
        }

        if guard_pos.y >= map.len() || guard_pos.x >= map[0].len() {
            break;
        }
    }

    println!("{}", visited_positions.len());
}