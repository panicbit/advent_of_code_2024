use std::collections::{HashMap, HashSet};
use std::iter;

use aoc::aoc;
use itertools::Itertools;

#[aoc(2024, 6, 2)]
fn main(input: &str) -> usize {
    let grid = parse_grid(input);

    looping_blockades(&grid).count()
}

fn looping_blockades(grid: &Grid) -> impl Iterator<Item = Position> + '_ {
    guard_path(grid)
        .map(|((x, y), (xd, yd))| (x + xd, y + yd))
        .filter(|position| {
            if grid.get(position) != Some(&'.') {
                return false;
            }

            let mut grid = grid.clone();

            grid.insert(*position, '#');

            path_contains_loop(guard_path(&grid))
        })
        .unique()
}

fn path_contains_loop(path: impl Iterator<Item = (Position, Direction)>) -> bool {
    let mut visited = HashSet::new();

    for path_item in path {
        if !visited.insert(path_item) {
            return true;
        }
    }

    false
}

fn guard_path(grid: &Grid) -> impl Iterator<Item = (Position, Direction)> + '_ {
    let mut directions = directions();
    let (mut x, mut y) = find_start_position(grid);
    let (mut xd, mut yd) = directions.next().unwrap();

    iter::from_fn(move || {
        if !grid.contains_key(&(x, y)) {
            return None;
        }

        let result = Some(((x, y), (xd, yd)));
        let next_position = (x + xd, y + yd);

        if grid.get(&next_position) == Some(&'#') {
            (xd, yd) = directions.next().unwrap();
            return result;
        }

        (x, y) = next_position;

        result
    })
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
