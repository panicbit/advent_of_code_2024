use std::collections::HashMap;
use std::convert::identity;
use std::iter;

use aoc::aoc;

#[aoc(2024, 4, 2)]
fn main(input: &str) -> usize {
    let grid = parse_grid(input);

    x_mas_occurrences(&grid).count()
}

type Grid = HashMap<(i32, i32), char>;
type Direction = (i32, i32);
type Position = (i32, i32);

const DOWN_RIGHT: Direction = (1, 1);
const UP_RIGHT: Direction = (1, -1);

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

fn x_mas_occurrences(grid: &Grid) -> impl Iterator<Item = Position> + '_ {
    grid.keys().copied().filter(|&(x, y)| {
        all([
            any([
                is_word_in_direction(grid, "MAS", (x, y), DOWN_RIGHT),
                is_word_in_direction(grid, "SAM", (x, y), DOWN_RIGHT),
            ]),
            any([
                is_word_in_direction(grid, "MAS", (x, y + 2), UP_RIGHT),
                is_word_in_direction(grid, "SAM", (x, y + 2), UP_RIGHT),
            ]),
        ])
    })
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

fn any(iter: impl IntoIterator<Item = bool>) -> bool {
    iter.into_iter().any(identity)
}

fn all(iter: impl IntoIterator<Item = bool>) -> bool {
    iter.into_iter().all(identity)
}
