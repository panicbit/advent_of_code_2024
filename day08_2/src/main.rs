use fnv::FnvHashMap;
use itertools::Itertools;

use aoc::aoc;

mod position;
use position::Position;

mod vector;

#[aoc(2024, 8, 2)]
fn main(input: &str) -> usize {
    let grid = parse_grid(input);
    let grouped_antennas = group_antennas_by_frequency(&grid);

    calculate_antinodes(&grid, &grouped_antennas)
        .filter(|position| grid.contains_key(position))
        .unique()
        .count()
}

type Grid = FnvHashMap<Position, char>;
type GroupedAntennas = FnvHashMap<char, Vec<Position>>;

fn calculate_antinodes<'a>(
    grid: &'a Grid,
    grouped_antennas: &'a GroupedAntennas,
) -> impl Iterator<Item = Position> + 'a {
    grouped_antennas
        .values()
        .flat_map(|antennas| calculate_antenna_list_antinodes(grid, antennas))
}

fn calculate_antenna_list_antinodes<'a>(
    grid: &'a Grid,
    antennas: &'a [Position],
) -> impl Iterator<Item = Position> + 'a {
    antennas
        .iter()
        .copied()
        .tuple_combinations()
        .flat_map(|pair| calculate_antenna_pair_antinodes(grid, pair))
}

fn calculate_antenna_pair_antinodes(grid: &Grid, (a, b): (Position, Position)) -> Vec<Position> {
    let mut positions = Vec::new();
    let mut vector = b - a;

    let mut antinode_position = a;

    while grid.contains_key(&antinode_position) {
        positions.push(antinode_position);
        antinode_position = antinode_position + vector;
    }

    vector = vector.flipped();
    antinode_position = a + vector;

    while grid.contains_key(&antinode_position) {
        positions.push(antinode_position);
        antinode_position = antinode_position + vector;
    }

    positions
}

fn group_antennas_by_frequency(grid: &Grid) -> GroupedAntennas {
    let mut grouped_antennas = GroupedAntennas::default();

    for (&position, &cell) in grid.iter() {
        if cell == '.' {
            continue;
        }

        grouped_antennas
            .entry(cell)
            .or_insert(Vec::new())
            .push(position);
    }

    grouped_antennas
}

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
