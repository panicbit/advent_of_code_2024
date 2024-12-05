use std::collections::HashMap;
use std::iter;

use aoc::aoc;
use itertools::Itertools;

#[aoc(2024, 4, 1)]
fn main(input: &str) -> usize {
    let grid = parse_grid(input);

    word_occurrences(&grid, "XMAS").count()
}

type Grid = HashMap<(i32, i32), char>;
type Direction = (i32, i32);
type Position = (i32, i32);

const DIRECTIONS: [Direction; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse_grid(input: &str) -> Grid {
    let mut grid = HashMap::new();

    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;

            grid.insert((x, y), ch);
        }
    }

    grid
}

fn word_occurrences<'a>(
    grid: &'a Grid,
    word: &'a str,
) -> impl Iterator<Item = (Position, Direction)> + 'a {
    grid.keys()
        .copied()
        .cartesian_product(DIRECTIONS)
        .filter(|(position, direction)| is_word_in_direction(grid, word, *position, *direction))
}

fn is_word_in_direction(grid: &Grid, word: &str, position: Position, direction: Direction) -> bool {
    word.chars()
        .zip(direction_chars(grid, position, direction))
        .filter(|(a, b)| a == b)
        .count()
        == word.len()
}

fn direction_chars(
    grid: &Grid,
    (mut x, mut y): Position,
    (xd, yd): Direction,
) -> impl Iterator<Item = char> + '_ {
    iter::from_fn(move || {
        let value = grid.get(&(x, y)).copied();

        x += xd;
        y += yd;

        value
    })
}
