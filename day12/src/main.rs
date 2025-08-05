use std::collections::{BTreeMap, HashSet};

use aoc::aoc;

#[aoc(2024, 12, 1)]
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
        let price = area * perimeter;

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

fn perimeter(region: &Region, garden: &Garden) -> usize {
    let plot = region.iter().next().unwrap();
    let label = garden.get(plot).unwrap();
    let mut perimeter = 0;

    for plot in region {
        for neighbour in neighbours(*plot) {
            if garden.get(&neighbour) == Some(label) {
                continue;
            }

            perimeter += 1;
        }
    }

    perimeter
}

fn neighbours((x, y): Plot) -> impl Iterator<Item = Plot> {
    [
        (x, y - 1), //
        (x, y + 1), //
        (x - 1, y), //
        (x + 1, y), //
    ]
    .into_iter()
}
