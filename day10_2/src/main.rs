use std::collections::{BTreeMap, BTreeSet};

use aoc::aoc;

#[aoc(2024, 10, 2)]
fn main(input: &str) -> usize {
    let mountain = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                ch.to_digit(10)
                    .map(|elevation| ((x as isize, y as isize), elevation))
            })
        })
        .collect::<Mountain>();

    let result = trails(&mountain)
        .map(|trail| paths(&mountain, trail))
        .sum::<usize>();

    result
}

type Mountain = BTreeMap<Pos, u32>;
type Pos = (isize, isize);
type Trail = Pos;

fn trails(mountain: &Mountain) -> impl Iterator<Item = Pos> + use<'_> {
    mountain
        .iter()
        .filter(|(_, elevation)| **elevation == 0)
        .map(|(pos, _)| *pos)
}

fn paths(mountain: &Mountain, trail: Trail) -> usize {
    let elevation = 0;

    assert_eq!(mountain.get(&trail), Some(&elevation));

    summits_rec(mountain, trail, elevation)
}

fn summits_rec(mountain: &Mountain, trail: Trail, elevation: u32) -> usize {
    if elevation == 9 {
        return 1;
    }

    let mut num_paths = 0;

    for (pos, next_elevation) in neighbours(mountain, trail) {
        if elevation + 1 != next_elevation {
            continue;
        }

        num_paths += summits_rec(mountain, pos, next_elevation);
    }

    num_paths
}

#[allow(clippy::identity_op)]
fn neighbours(
    mountain: &Mountain,
    (x, y): (isize, isize),
) -> impl Iterator<Item = ((isize, isize), u32)> + use<'_> {
    [
        (x - 1, y - 0),
        (x + 0, y - 1),
        (x + 0, y + 1),
        (x + 1, y + 0),
    ]
    .into_iter()
    .filter_map(|pos| mountain.get_key_value(&pos))
    .map(|(pos, elevation)| (*pos, *elevation))
}
