use std::collections::BTreeMap;

use aoc::aoc;

#[aoc(2024, 15, 1)]
fn main(input: &str) -> isize {
    let (grid, dirs) = input.split_once("\n\n").unwrap();
    let mut grid = grid
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as isize, y as isize), Cell::parse(ch)))
        })
        .collect::<Grid>();
    let dirs = dirs
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(Dir::parse)
        .collect::<Vec<_>>();

    for dir in dirs {
        let robot = find_robot(&grid);

        move_cell(&mut grid, robot, dir);
    }

    let result = boxes(&grid).map(|(x, y)| x + 100 * y).sum::<isize>();

    result
}

fn find_robot(grid: &Grid) -> Pos {
    *grid.iter().find(|(_, cell)| cell.is_robot()).unwrap().0
}

fn boxes(grid: &Grid) -> impl Iterator<Item = Pos> + use<'_> {
    grid.iter()
        .filter(|(_, cell)| cell.is_box())
        .map(|(pos, _)| *pos)
}

fn move_cell(grid: &mut Grid, pos: Pos, dir: Dir) -> bool {
    let Some(cell) = grid.get(&pos).copied() else {
        unreachable!("should not try to push outside of grid");
    };

    let next_pos = dir.apply_to(pos);

    match cell {
        Cell::Empty => true,
        Cell::Wall => false,
        Cell::Robot | Cell::Box => {
            if move_cell(grid, next_pos, dir) {
                grid.insert(pos, Cell::Empty);
                grid.insert(next_pos, cell);
                true
            } else {
                false
            }
        }
    }
}

type Pos = (isize, isize);
type Grid = BTreeMap<Pos, Cell>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Robot,
    Box,
}

impl Cell {
    fn parse(cell: char) -> Self {
        match cell {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '@' => Self::Robot,
            'O' => Self::Box,
            ch => unreachable!("ch is {ch:?}"),
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    fn is_wall(&self) -> bool {
        matches!(self, Self::Wall)
    }

    fn is_robot(&self) -> bool {
        matches!(self, Self::Robot)
    }

    fn is_box(&self) -> bool {
        matches!(self, Self::Box)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn parse(dir: char) -> Self {
        match dir {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unimplemented!("dir: {dir:?}"),
        }
    }

    fn apply_to(&self, (x, y): Pos) -> Pos {
        match self {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        }
    }
}
