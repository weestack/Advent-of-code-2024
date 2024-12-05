use std::fs;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let data = fs::read_to_string("input/day4.txt").expect("Unable to read file");
    let data = data.lines().map(|line| line.to_string()).collect::<Vec<String>>();
    let reversed = reverse_strings(&data);
    let transposed = transpose_strings(&data);
    let transposed_reversed = reverse_strings(&reversed);

    let diagonals = extract_diagonals(&data);
    let reversed_diagonals = extract_diagonals(&reversed);
    let mut count = 0;
    count += count_xmas(&data);
    count += count_xmas(&reversed);
    count += count_xmas(&transposed);
    count += count_xmas(&transposed_reversed);
    count += count_xmas(&diagonals);
    count += count_xmas(&reversed_diagonals);

    let sol1: u64 = count as u64;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn count_xmas(lines: &Vec<String>) -> u32 {
    let mut count = 0;
    for line in lines {
        for i in 0..line.len() - 4 {
            if line[i..i+4] == "XMAS".to_string() {
                count += 1;
            }
        }
    }

    count
}

fn reverse_strings(lines: &Vec<String>) -> Vec<String> {
    let reversed: Vec<String> = lines.iter()
        .map(|word| word.chars().rev().collect::<String>() )
        .collect();
    reversed
}

fn transpose_strings(lines: &Vec<String>) -> Vec<String> {
    // Initialize transposed with empty strings of appropriate length
    let mut transposed: Vec<String> = vec![String::new(); lines[0].len()];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            transposed[i].push(c);
        }
    }

    transposed
}

// Extract all diagonals (top-left to bottom-right and top-right to bottom-left)
fn extract_diagonals(lines: &Vec<String>) -> Vec<String> {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut diagonals = Vec::new();

    // Top-left to bottom-right
    for d in 0..(rows + cols - 1) {
        let mut diagonal = String::new();
        for i in 0..=d {
            let j = d - i;
            if i < rows && j < cols {
                diagonal.push(lines[i].chars().nth(j).unwrap());
            }
        }
        if diagonal.len() >= 4 {
            diagonals.push(diagonal);
        }
    }

    // Top-right to bottom-left
    for d in 0..(rows + cols - 1) {
        let mut diagonal = String::new();
        for i in 0..=d {
            // Ensure j is always valid
            if d >= i && d - i < cols {
                let j = cols - 1 - (d - i);
                if i < rows && j < cols {
                    diagonal.push(lines[i].chars().nth(j).unwrap());
                }
            }
        }
        if diagonal.len() >= 4 {
            diagonals.push(diagonal);
        }
    }

    diagonals
}
