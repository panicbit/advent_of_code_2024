from calendar import c
from dataclasses import dataclass
from enum import Enum, IntEnum, auto
from operator import attrgetter, methodcaller
from typing import Dict, Generator, List, Never, Set, Tuple

input = open("input.txt").read()

type Plot = Tuple[int, int]
type Label = str
type Garden = Dict[Plot, Label]

def main() -> None:
    print(f"part 1: {part1(input)}")
    print(f"part 2: {part2(input)}")

def part1(input: str) -> int:
    garden = parse_input(input)
    regions = find_regions(garden)
    total = 0

    for region in regions:
        area = len(region.plots)
        circumference = len(region.perimeter())
        price = area * circumference
        total += price

    return total

def part2(input: str) -> int:
    garden = parse_input(input)
    regions = find_regions(garden)
    total = 0

    for region in regions:
        area = len(region.plots)
        edges = region.edges()
        price = area * len(edges)
        total += price

    return total

@dataclass
class Region:
    label: str
    plots: Set[Plot]

    def perimeter(self) -> List["Side"]:
        sides = []

        for plot in self.plots:
            for neighbour, dir in neighbours_with_dirs(plot):
                if neighbour not in self.plots:
                    sides.append(Side(dir, plot))

        return sides
    
    def edges(self) -> List[List["Side"]]:
        def inner() -> Generator[List[Side]]:
            sides = self.perimeter()
            sides = sorted(sides, key = methodcaller('axis_coord'))
            sides = sorted(sides, key = methodcaller('cross_axis_coord'))
            sides = sorted(sides, key = attrgetter('dir'))

            edge: List[Side] = []

            for side in sides:
                if not edge:
                    edge.append(side)
                    continue

                last = edge[-1];
                dirs_match = last.dir == side.dir
                cross_axes_match = last.cross_axis_coord() == side.cross_axis_coord()
                axes_consecutive = last.axis_coord() + 1 == side.axis_coord()

                if all([dirs_match, cross_axes_match, axes_consecutive]):
                    edge.append(side)
                else:
                    yield edge
                    edge = [side]

            yield edge
        return list(inner())

@dataclass
class Side:
    dir: "Dir"
    plot: Plot

    def axis_coord(self) -> int:
        return self.dir.axis().get_coord(self.plot)

    def cross_axis_coord(self) -> int:
        return self.dir.cross_axis().get_coord(self.plot)

class Dir(IntEnum):
    Up = auto()
    Down = auto()
    Left = auto()
    Right = auto()

    def axis(self) -> "Axis":
        match self:
            case Dir.Up | Dir.Down: return Axis.Vertical
            case Dir.Left | Dir.Right: return Axis.Horizontal
    
    def cross_axis(self) -> "Axis":
        match self:
            case Dir.Up | Dir.Down: return Axis.Horizontal
            case Dir.Left | Dir.Right: return Axis.Vertical

class Axis(Enum):
    Horizontal = auto()
    Vertical = auto()

    def get_coord(self, plot: Plot) -> int:
        x, y = plot

        match self:
            case Axis.Horizontal: return y
            case Axis.Vertical: return x

def parse_input(input: str) -> Garden:
    garden = {}

    for y, line in enumerate(input.splitlines()):
        for x, label in enumerate(line):
            garden[(x, y)] = label

    return garden

def find_regions(garden: Garden) -> List[Region]:
    remaining = dict(garden)
    regions = []

    while remaining:
        plot, label = next(iter(remaining.items()))
        region = find_region(plot, label, garden)

        regions.append(region)

        for plot in region.plots:
            remaining.pop(plot)
        
    
    return regions

def find_region(plot: Plot, label: Label, garden: Garden) -> Region:
    plots: Set[Plot] = set()

    find_region_rec(plot, label, garden, plots)

    return Region(label, plots)

def find_region_rec(plot: Plot, label: Label, garden: Garden, plots: Set[Plot]) -> None:
    if plot in plots:
        return

    if garden.get(plot) != label:
        return

    plots.add(plot)

    for neighbour in neighbours(plot): 
        find_region_rec(neighbour, label, garden, plots)

def neighbours(plot: Plot) -> List[Plot]:
    x, y = plot

    return [
        (x, y -1),
        (x, y + 1),
        (x - 1, y),
        (x + 1, y),
    ]

def neighbours_with_dirs(plot: Plot) -> List[Tuple[Plot, Dir]]:
    x, y = plot

    return [
        ((x, y -1), Dir.Up),
        ((x, y + 1), Dir.Down),
        ((x - 1, y), Dir.Left),
        ((x + 1, y), Dir.Right),
    ]

if __name__ == "__main__":
    main()
