use std::collections::HashMap;

use aoc_2024::AoCResult;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let vals = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let l1 = vals.iter().map(|row| row[0]).collect::<Vec<_>>();
    let l2 = vals.iter().map(|row| row[1]).collect::<Vec<_>>();
    return (l1, l2);
}

pub fn part_one(input: &str) -> AoCResult {
    let (mut l1, mut l2) = parse_input(input);

    l1.sort();
    l1.reverse();
    l2.sort();
    l2.reverse();

    let diff: i32 = (0..l1.len())
        .map(|_i| (l1.pop().unwrap() - l2.pop().unwrap()).abs())
        .sum();

    AoCResult::Int(diff as i64)
}

pub fn part_two(input: &str) -> AoCResult {
    let (l1, l2) = parse_input(input);

    let mut histo: HashMap<i32, i32> = HashMap::new();
    l2.iter().for_each(|v| *histo.entry(*v).or_insert(0) += 1);
    let res: i32 = l1.iter().map(|v| v * histo.get(&v).unwrap_or(&0)).sum();

    AoCResult::Int(res as i64)
}
