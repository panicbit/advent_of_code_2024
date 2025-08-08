from dataclasses import dataclass
from enum import Enum, IntEnum, auto
from operator import attrgetter, methodcaller
from typing import Dict, Generator, List, Set, Tuple

input = open("input.txt").read()

type Plot = Tuple[int, int]
type Label = str

def main() -> None:
    garden = Garden.parse(input)
    regions = garden.regions()

    print(f"part 1: {part1(regions)}")
    print(f"part 2: {part2(regions)}")

def part1(regions: List["Region"]) -> int:
    total = 0

    for region in regions:
        area = len(region.plots)
        perimeter = len(region.perimeter())
        price = area * perimeter
        total += price

    return total

def part2(regions: List["Region"]) -> int:
    total = 0

    for region in regions:
        area = len(region.plots)
        sides = len(region.edges())
        price = area * sides
        total += price

    return total

@dataclass
class Garden:
    plot_labels: Dict[Plot, Label]
    
    @staticmethod
    def parse(garden: str) -> "Garden":
        plot_labels = {}

        for y, line in enumerate(garden.splitlines()):
            for x, label in enumerate(line):
                plot_labels[(x, y)] = label

        return Garden(plot_labels)
    
    def regions(self) -> List["Region"]:
        remaining = dict(self.plot_labels)
        regions = []

        while remaining:
            plot, label = next(iter(remaining.items()))
            region = self.region(plot, label)

            regions.append(region)

            for plot in region.plots:
                remaining.pop(plot)
            
        
        return regions
    
    def region(self, plot: Plot, label: Label) -> "Region":
        plots: Set[Plot] = set()
        to_check: Set[Plot] = set([plot])

        while to_check:
            plot = to_check.pop()

            if plot in plots:
                continue

            if self.plot_labels.get(plot) != label:
                continue

            plots.add(plot)

            for neighbour in neighbours(plot): 
                to_check.add(neighbour)

        return Region(label, plots)

@dataclass
class Region:
    label: str
    plots: Set[Plot]

    def perimeter(self) -> List["Edge"]:
        edges = []

        for plot in self.plots:
            for neighbour, dir in neighbours_with_dirs(plot):
                if neighbour not in self.plots:
                    edges.append(Edge(dir, plot))

        return edges
    
    def edges(self) -> List[List["Edge"]]:
        def inner() -> Generator[List[Edge]]:
            edges = self.perimeter()
            edges = sorted(edges, key = methodcaller('axis_coord'))
            edges = sorted(edges, key = methodcaller('cross_axis_coord'))
            edges = sorted(edges, key = attrgetter('dir'))

            side: List[Edge] = []

            for edge in edges:
                if not side:
                    side.append(edge)
                    continue

                last = side[-1];
                dirs_match = last.dir == edge.dir
                cross_axes_match = last.cross_axis_coord() == edge.cross_axis_coord()
                axes_consecutive = last.axis_coord() + 1 == edge.axis_coord()

                if all([dirs_match, cross_axes_match, axes_consecutive]):
                    side.append(edge)
                else:
                    yield side
                    side = [edge]

            yield side
        return list(inner())

@dataclass
class Edge:
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

def neighbours(plot: Plot) -> List[Plot]:
    return list(neighbour for neighbour, _ in neighbours_with_dirs(plot))

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
