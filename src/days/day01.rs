use std::collections::HashMap;
use std::fs;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {

    let sol1: u64 = part1();
    let sol2: u64 = part2();

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1() -> u64 {
    let data = fs::read_to_string("input/day1.txt").expect("Unable to read file");
    let mut lines = data.lines();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines.by_ref() {
        let numbers:Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }
    left.sort();
    right.sort();

    let difference: Vec<i32> = left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();
    println!("diff: {:?} ", difference);

    let total = difference.iter().sum::<i32>();
    println!("sum: {:?} ", total);
    total as u64
}

fn part2() -> u64 {
    let data = fs::read_to_string("input/day1.txt").expect("Unable to read file");
    let mut lines = data.lines();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines.by_ref() {
        let numbers:Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }
    left.sort();
    right.sort();

    let counts: HashMap<&i32, i32> = left
        .iter()
        .map(|a| (a, right.iter().filter(|b| **b == *a).count() as i32))
        .collect();


    // Calculate the sums directly
    let total_sum: i32 = counts
        .iter()
        .map(|(&k, &v)| k * v)
        .sum();

    println!("Part 2: {:?}", total_sum);
    total_sum as u64
}