use std::fs;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use regex::Regex;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let data = fs::read_to_string("input/day3.txt").expect("Unable to read file");
    let part1_re = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
    let part2_re = Regex::new(r"mul\((\d+)\,(\d+)\)|do\(\)|don\'t\(\)").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    for cap in part1_re.captures_iter(data.as_str()) {
        let mul1 = cap[1].parse::<i32>().unwrap();
        let mul2 = cap[2].parse::<i32>().unwrap();
        part1 += mul1 * mul2;
    }

    let mut follow_instructions = true;
    for cap in part2_re.captures_iter(data.as_str()) {
        let instruction = &cap[0];
        if instruction == "do()" {
            follow_instructions = true;
            continue;
        } else if instruction == "don't()" {
            follow_instructions = false;
            continue;
        }

        if follow_instructions && &instruction[..3] == "mul" {
            let mul1 = cap[1].parse::<i32>().unwrap();
            let mul2 = cap[2].parse::<i32>().unwrap();
            part2 += mul1 * mul2;
        }
    }


    (Solution::from(part1 as u64), Solution::from(part2 as u64))
}