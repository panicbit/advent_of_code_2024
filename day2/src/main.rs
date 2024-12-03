use aoc::aoc;
use itertools::Itertools;

#[aoc(2024, 2, 1)]
fn main(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            line.split(" ")
                .map(|level| level.parse::<i32>().unwrap())
                .tuple_windows()
                .map(|(a, b)| {
                    if !(1..=3).contains(&(a - b).abs()) {
                        return None;
                    }

                    if a < b {
                        Some(Gradient::Ascending)
                    } else {
                        Some(Gradient::Descending)
                    }
                })
                .tuple_windows()
                .all(|(a, b)| match (a, b) {
                    (Some(a), Some(b)) => a == b,
                    (_, _) => false,
                })
        })
        .count()
}

#[derive(PartialEq, Clone)]
enum Gradient {
    Ascending,
    Descending,
}
