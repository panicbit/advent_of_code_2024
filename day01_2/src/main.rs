use std::collections::HashMap;

use aoc::aoc;

#[aoc(2024, 1, 2)]
fn main(input: &str) -> i64 {
    let mut left = Vec::new();
    let mut right = HashMap::<i64, i64>::new();

    for line in input.lines() {
        let mut parts = line.split("   ");

        let a: i64 = parts.next().unwrap().parse().unwrap();
        let b: i64 = parts.next().unwrap().parse().unwrap();

        left.push(a);
        *right.entry(b).or_default() += 1;
    }

    let score: i64 = left
        .into_iter()
        .map(|a| {
            let occurrences = right.get(&a).copied().unwrap_or_default();

            a * occurrences
        })
        .sum();

    score
}
