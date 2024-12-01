use aoc::aoc;

#[aoc(2024, 1, 1)]
fn main(input: &str) -> i64 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut parts = line.split("   ");

        let a: i64 = parts.next().unwrap().parse().unwrap();
        let b: i64 = parts.next().unwrap().parse().unwrap();

        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    let sum: i64 = left
        .into_iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum();

    sum
}
