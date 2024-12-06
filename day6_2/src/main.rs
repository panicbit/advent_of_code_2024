use fnv::{FnvHashMap, FnvHashSet};
use std::iter;

use aoc::aoc;
use itertools::Itertools;

mod position;
use position::Position;

mod direction;
use direction::Direction;

#[aoc(2024, 6, 2)]
fn main(input: &str) -> usize {
    let grid = parse_grid(input);

    looping_blockades(&grid).count()
}

type Grid = FnvHashMap<Position, char>;

fn parse_grid(input: &str) -> Grid {
    let mut grid = FnvHashMap::default();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let position = Position(x as i32, y as i32);

            grid.insert(position, ch);
        }
    }

    grid
}

fn looping_blockades(grid: &Grid) -> impl Iterator<Item = Position> + '_ {
    guard_path(grid)
        .map(|(position, direction)| position + direction)
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

fn guard_path(grid: &Grid) -> impl Iterator<Item = (Position, Direction)> + '_ {
    let mut position = find_start_position(grid);
    let mut direction = Direction::Up;

    iter::from_fn(move || {
        if !grid.contains_key(&position) {
            return None;
        }

        let result = Some((position, direction));
        let next_position = position + direction;

        if grid.get(&next_position) == Some(&'#') {
            direction.rotate_right();
            return result;
        }

        position = next_position;

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

fn path_contains_loop(path: impl Iterator<Item = (Position, Direction)>) -> bool {
    let mut visited = FnvHashSet::default();

    for path_item in path {
        if !visited.insert(path_item) {
            return true;
        }
    }

    false
}
