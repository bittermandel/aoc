use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn test(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut leftvec = Vec::with_capacity(input.lines().count());
    let mut rightvec = Vec::with_capacity(input.lines().count());

    for row in input.lines() {
        let (left, right) = row.split_once("   ").unwrap();
        leftvec.push(left.parse::<i32>().unwrap());
        rightvec.push(right.parse::<i32>().unwrap());
    }

    leftvec.sort();
    rightvec.sort();

    (leftvec, rightvec)
}

#[aoc(day1, part1)]
fn part1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut distance = 0;

    for n in 0..input.0.len() {
        distance += (input.0[n] - input.1[n]).abs();
    }

    distance
}

#[aoc(day1, part2)]
fn part2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut similarity_score = 0;

    let mut num_count_right: HashMap<i32, i32> = HashMap::new();

    for value in &input.1 {
        num_count_right
            .entry(*value)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    for value in &input.0 {
        let right_value = num_count_right.get(value);
        if let Some(right_value) = right_value {
            similarity_score += value * right_value;
        }
    }

    similarity_score
}
