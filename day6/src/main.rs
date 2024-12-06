use std::collections::HashMap;
use std::iter;

use aoc::aoc;
use itertools::Itertools;

#[aoc(2024, 6, 1)]
fn main(input: &str) -> usize {
    let grid = parse_grid(input);

    guard_path(&grid).unique().count()
}

fn guard_path(grid: &Grid) -> impl Iterator<Item = Position> + '_ {
    let mut directions = directions();
    let (mut x, mut y) = find_start_position(grid);
    let (mut xd, mut yd) = directions.next().unwrap();

    iter::once((x, y)).chain(iter::from_fn(move || loop {
        let next_position = (x + xd, y + yd);

        let cell = grid.get(&next_position)?;

        if *cell == '#' {
            (xd, yd) = directions.next().unwrap();
            continue;
        }

        (x, y) = next_position;

        return Some(next_position);
    }))
}

fn find_start_position(grid: &Grid) -> Position {
    grid.iter()
        .find(|(_, cell)| **cell == '^')
        .map(|(position, _)| position)
        .copied()
        .unwrap()
}

type Grid = HashMap<(i32, i32), char>;
type Position = (i32, i32);
type Direction = (i32, i32);

fn parse_grid(input: &str) -> Grid {
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;

            grid.insert((x, y), ch);
        }
    }

    grid
}

fn directions() -> impl Iterator<Item = Direction> {
    [(0, -1), (1, 0), (0, 1), (-1, 0)].into_iter().cycle()
}
