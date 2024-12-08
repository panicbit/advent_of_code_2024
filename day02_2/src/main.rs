use aoc::aoc;
use itertools::Itertools;

#[aoc(2024, 2, 2)]
fn main(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels = line
                .split(" ")
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if levels_safe(&levels) {
                return true;
            }

            for (i, _) in levels.iter().enumerate() {
                let mut levels = levels.clone();

                levels.remove(i);

                if levels_safe(&levels) {
                    return true;
                }
            }

            false
        })
        .count()
}

fn levels_safe(levels: &[i32]) -> bool {
    levels
        .iter()
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
}

#[derive(PartialEq, Clone)]
enum Gradient {
    Ascending,
    Descending,
}
