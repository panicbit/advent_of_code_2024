use std::collections::BTreeMap;

use aoc::aoc;

#[aoc(2024, 15, 2)]
fn main(input: &str) -> isize {
    let (grid, dirs) = input.split_once("\n\n").unwrap();
    let mut grid = grid
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, ch)| {
                let y = y as isize;
                let x = 2 * x as isize;
                let cell = Cell::parse(ch);

                [
                    ((x, y), Cell::parse(ch)), //
                    ((x + 1, y), cell.twin()), //
                ]
            })
        })
        .collect::<Grid>();

    let dirs = dirs
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(Dir::parse)
        .collect::<Vec<_>>();

    for dir in dirs {
        let robot = find_robot(&grid);

        if can_move_cell(&grid, robot, dir) {
            move_cell(&mut grid, robot, dir);
        }
    }

    let result = boxes(&grid).map(|(x, y)| x + 100 * y).sum::<isize>();

    result
}

fn print_grid(grid: &Grid) {
    let width = grid.keys().map(|(x, _)| *x).max().unwrap();
    let height = grid.keys().map(|(_, y)| *y).max().unwrap();

    for y in 0..=height {
        for x in 0..=width {
            let Some(cell) = grid.get(&(x, y)) else {
                print!("?");
                continue;
            };

            match cell {
                Cell::Empty => print!("."),
                Cell::Wall => print!("#"),
                Cell::Robot => print!("@"),
                Cell::BoxLeft => print!("["),
                Cell::BoxRight => print!("]"),
            }
        }
        println!()
    }
}

fn find_robot(grid: &Grid) -> Pos {
    *grid.iter().find(|(_, cell)| cell.is_robot()).unwrap().0
}

fn boxes(grid: &Grid) -> impl Iterator<Item = Pos> + use<'_> {
    grid.iter()
        .filter(|(_, cell)| cell.is_box_left())
        .map(|(pos, _)| *pos)
}

fn can_move_cell(grid: &Grid, pos: Pos, dir: Dir) -> bool {
    let Some(cell) = grid.get(&pos).copied() else {
        unreachable!("should not try to push outside of grid");
    };

    let next_pos = dir.apply_to(pos);

    match cell {
        Cell::Empty => true,
        Cell::Wall => false,
        Cell::Robot => can_move_cell(grid, next_pos, dir),
        Cell::BoxLeft | Cell::BoxRight => {
            if dir.is_horizontal() {
                return can_move_cell(grid, next_pos, dir);
            }

            let other_dir = if cell.is_box_left() {
                Dir::Right
            } else {
                Dir::Left
            };
            let other_pos = other_dir.apply_to(pos);
            let other_next_pos = dir.apply_to(other_pos);
            let can_move = can_move_cell(grid, next_pos, dir);
            let can_move_other = can_move_cell(grid, other_next_pos, dir);

            can_move && can_move_other
        }
    }
}

fn move_cell(grid: &mut Grid, pos: Pos, dir: Dir) {
    let Some(cell) = grid.get(&pos).copied() else {
        unreachable!("should not try to push outside of grid");
    };

    let next_pos = dir.apply_to(pos);

    match cell {
        Cell::Empty => {}
        Cell::Wall => {}
        Cell::Robot => {
            move_cell(grid, next_pos, dir);
            grid.insert(next_pos, cell);
            grid.insert(pos, Cell::Empty);
        }
        Cell::BoxLeft | Cell::BoxRight => {
            if dir.is_horizontal() {
                move_cell(grid, next_pos, dir);
                grid.insert(next_pos, cell);
                grid.insert(pos, Cell::Empty);
                return;
            }

            let other_dir = if cell.is_box_left() {
                Dir::Right
            } else {
                Dir::Left
            };
            let other_pos = other_dir.apply_to(pos);
            let other_next_pos = dir.apply_to(other_pos);
            let other_cell = cell.twin();

            move_cell(grid, next_pos, dir);
            move_cell(grid, other_next_pos, dir);

            grid.insert(next_pos, cell);
            grid.insert(other_next_pos, other_cell);

            grid.insert(pos, Cell::Empty);
            grid.insert(other_pos, Cell::Empty);
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
    BoxLeft,
    BoxRight,
}

impl Cell {
    fn parse(cell: char) -> Self {
        match cell {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '@' => Self::Robot,
            'O' => Self::BoxLeft,
            ch => unreachable!("ch is {ch:?}"),
        }
    }

    fn twin(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Wall => Self::Wall,
            Self::Robot => Self::Empty,
            Self::BoxLeft => Self::BoxRight,
            Self::BoxRight => Self::BoxLeft,
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

    fn is_box_left(&self) -> bool {
        matches!(self, Self::BoxLeft)
    }

    fn is_box_right(&self) -> bool {
        matches!(self, Self::BoxRight)
    }

    fn is_box(&self) -> bool {
        matches!(self, Self::BoxLeft | Self::BoxRight)
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

    fn is_horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}
