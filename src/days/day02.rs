use std::fs;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let data = fs::read_to_string("input/day2.txt").expect("Unable to read file");
    let mut lines = data.lines();
    let mut safe_reports = 0;
    let mut safe_reports2 = 0;
    for line in lines.by_ref() {
        let mut report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if is_safe(&report) {
            println!("{:?}", report);
            safe_reports += 1;
            safe_reports2 += 1;
        }

        if !is_safe(&report) {
            for i in 0..report.len() {
                let mut dampener_report = report.clone();
                dampener_report.remove(i);
                if is_safe(&dampener_report) {
                    safe_reports2 += 1;
                    break;
                }
            }
        }
    }


    (Solution::from(safe_reports as u64), Solution::from(safe_reports2 as u64))
}

fn is_safe(input: &Vec<i32>) -> bool {
    let mut report = input.clone();
    let mut prev: i32 = report.remove(0);
    let mut direction = -3;

    while !report.is_empty() {
        let next = report.remove(0);
        let delta_next = prev - next;
        if direction == -3 {
            direction = if prev > next { 1 } else { -1 };
        }

        if delta_next == 0 {
            return false;
        }

        if direction == 1 && delta_next < 0 || direction == -1 && delta_next > 0 {
            return false;
        }

        if direction == 1 && delta_next > 3 {
            return false;
        }

        if direction == -1 && delta_next < -3 {
            return false;
        }
        prev = next;
    }
    true
}