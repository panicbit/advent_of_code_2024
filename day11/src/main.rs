#![recursion_limit = "256"]

use std::{iter, option};

use aoc::aoc;

type N = u64;
type F = f64;

#[aoc(2024, 11, 1)]
fn main(input: &str) -> usize {
    // let input = "0 1 10 99 999";
    let stones = input.trim().split(' ').map(|n| n.parse::<N>().unwrap());

    let stones = blink(blink(blink(blink(blink(blink(blink(blink(blink(
        blink(blink(blink(blink(blink(blink(blink(blink(blink(
            blink(blink(blink(blink(blink(blink(blink(stones))))))),
        ))))))))),
    )))))))));

    stones.count()
}

fn blink(stones: impl Iterator<Item = N>) -> impl Iterator<Item = N> {
    stones.flat_map(|stone| {
        if stone == 0 {
            iter_one(1)
        } else if is_even(num_digits(stone)) {
            let (left, right) = split(stone);

            iter_two(left, right)
        } else {
            iter_one(stone * 2024)
        }
    })
}

fn iter_one(n: N) -> iter::Chain<option::IntoIter<N>, option::IntoIter<N>> {
    Some(n).into_iter().chain(None)
}

fn iter_two(a: N, b: N) -> iter::Chain<option::IntoIter<N>, option::IntoIter<N>> {
    Some(a).into_iter().chain(Some(b))
}

fn num_digits(n: N) -> N {
    if n == 0 {
        return 1;
    }

    ((n + 1) as F).log10().ceil() as N
}

fn is_even(n: N) -> bool {
    n % 2 == 0
}

fn split(n: N) -> (N, N) {
    let n = n.to_string();
    let (left, right) = n.split_at(n.len() / 2);
    let left = left.parse::<N>().unwrap();
    let right = right.parse::<N>().unwrap();

    (left, right)
}
