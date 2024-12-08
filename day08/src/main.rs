use fnv::FnvHashMap;
use itertools::Itertools;

use aoc::aoc;

mod position;
use position::Position;

mod vector;

#[aoc(2024, 8, 1)]
fn main(input: &str) -> usize {
    let grid = parse_grid(input);
    let grouped_antennas = group_antennas_by_frequency(&grid);

    calculate_antinodes(&grouped_antennas)
        .filter(|position| grid.contains_key(position))
        .unique()
        .count()
}

type Grid = FnvHashMap<Position, char>;
type GroupedAntennas = FnvHashMap<char, Vec<Position>>;

fn calculate_antinodes(grouped_antennas: &GroupedAntennas) -> impl Iterator<Item = Position> + '_ {
    grouped_antennas
        .values()
        .flat_map(|antennas| calculate_antenna_list_antinodes(antennas))
}

fn calculate_antenna_list_antinodes(antennas: &[Position]) -> impl Iterator<Item = Position> + '_ {
    antennas
        .iter()
        .copied()
        .tuple_combinations()
        .flat_map(calculate_antenna_pair_antinodes)
}

fn calculate_antenna_pair_antinodes((a, b): (Position, Position)) -> [Position; 2] {
    let vector = b - a;

    let antinode_a = a + vector.flipped();
    let antinode_b = b + vector;

    [antinode_a, antinode_b]
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
