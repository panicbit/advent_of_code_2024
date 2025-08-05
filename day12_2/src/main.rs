use std::collections::{BTreeMap, HashSet};

use aoc::aoc;
use itertools::Itertools;

#[aoc(2024, 12, 2)]
fn main(input: &str) -> usize {
    let garden = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(x, label)| ((x as isize, y as isize), label))
        })
        .collect::<Garden>();

    let mut remaining_plots = garden.clone();
    let mut result = 0;

    while !remaining_plots.is_empty() {
        let (&plot, &label) = remaining_plots.iter().next().unwrap();
        let region = find_region(plot, label, &garden);
        let area = region.len();
        let perimeter = perimeter(&region, &garden);
        let sides = sides(&perimeter, &garden);
        let price = area * sides;

        result += price;

        for plot in region {
            remaining_plots.remove(&plot);
        }
    }

    result
}

type Plot = (isize, isize);
type Garden = BTreeMap<Plot, char>;
type Region = HashSet<Plot>;
type Label = char;

fn find_region(plot: Plot, label: Label, garden: &Garden) -> Region {
    let mut region = HashSet::new();

    find_region_rec(plot, &mut region, label, garden);

    region
}

fn find_region_rec(plot: Plot, region: &mut Region, label: Label, garden: &Garden) {
    if region.contains(&plot) {
        return;
    }

    if garden.get(&plot) != Some(&label) {
        return;
    }

    region.insert(plot);

    for neighbour in neighbours(plot) {
        find_region_rec(neighbour, region, label, garden);
    }
}

fn perimeter(region: &Region, garden: &Garden) -> HashSet<Plot> {
    let plot = region.iter().next().unwrap();
    let label = garden.get(plot).unwrap();
    let mut perimeter = HashSet::new();

    for plot in region {
        for neighbour in neighbours(*plot) {
            if garden.get(&neighbour) == Some(label) {
                continue;
            }

            perimeter.insert(*plot);
        }
    }

    perimeter
}

fn sides(perimeter: &Region, garden: &Garden) -> usize {
    let plot = perimeter.iter().next().unwrap();
    let label = *garden.get(plot).unwrap();

    perimeter
        .iter()
        .flat_map(|&plot| {
            neighbours_with_directions(plot)
                .filter(|(neighbour, _)| {
                    garden
                        .get(neighbour)
                        .is_none_or(|neighbour_label| label != *neighbour_label)
                })
                .map(move |(_, dir)| Side { plot, dir })
        })
        .sorted_by(|a, b| {
            a.dir
                .cmp(&b.dir)
                .then(a.cross_axis_coord().cmp(&b.cross_axis_coord()))
                .then(a.axis_coord().cmp(&b.axis_coord()))
        })
        .coalesce(|a, b| {
            if a.dir != b.dir {
                return Err((a, b));
            }

            if a.cross_axis_coord() != b.cross_axis_coord() {
                return Err((a, b));
            }

            if a.axis_coord() + 1 != b.axis_coord() {
                return Err((a, b));
            }

            Ok(b)
        })
        .count()
}

fn neighbours((x, y): Plot) -> impl Iterator<Item = Plot> {
    [
        (x, y - 1), // up
        (x, y + 1), // down
        (x - 1, y), // left
        (x + 1, y), // right
    ]
    .into_iter()
}

fn neighbours_with_directions((x, y): Plot) -> impl Iterator<Item = (Plot, Dir)> {
    [
        ((x, y - 1), Dir::Up),
        ((x, y + 1), Dir::Down),
        ((x - 1, y), Dir::Left),
        ((x + 1, y), Dir::Right),
    ]
    .into_iter()
}

#[derive(Debug)]
struct Side {
    plot: Plot,
    dir: Dir,
}

impl Side {
    fn axis_coord(&self) -> isize {
        self.dir.axis().get_coord(self.plot)
    }

    fn cross_axis_coord(&self) -> isize {
        self.dir.cross_axis().get_coord(self.plot)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn axis(&self) -> Axis {
        match self {
            Dir::Up | Dir::Down => Axis::Vertical,
            Dir::Left | Dir::Right => Axis::Horizontal,
        }
    }

    fn cross_axis(&self) -> Axis {
        match self {
            Dir::Up | Dir::Down => Axis::Horizontal,
            Dir::Left | Dir::Right => Axis::Vertical,
        }
    }
}

enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    fn get_coord(&self, (x, y): Plot) -> isize {
        match self {
            Axis::Horizontal => y,
            Axis::Vertical => x,
        }
    }
}
