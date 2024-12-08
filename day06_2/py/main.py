from copy import deepcopy
from pathlib import Path
from typing import Dict, Generator, Iterable, Tuple
from position import Position
from direction import Direction

Grid = Dict[Position, str]

def main():
    input = Path('input.txt').read_text()
    grid = parse_grid(input)
    result = len(list(looping_blockades(grid)))

    print(result)
    
def looping_blockades(grid: Grid) -> Generator[Position, None, None]:
    plan = deepcopy(grid)
    visited = set()

    for position, _ in guard_path(grid):
        if position in visited:
            continue

        visited.add(position)

        if grid.get(position) != '.':
            continue

        plan[position] = '#'
        
        if path_contains_loop(guard_path(plan)):
            yield position

        plan[position] = '.'
    

def guard_path(grid: Grid) -> Generator[Tuple[Position, Direction], None, None]:
    position = find_start_position(grid)
    direction = Direction.Up

    while position in grid:
        yield (position, direction)

        next_position = position + direction

        if grid.get(next_position) == '#':
            direction = direction.rotated_right()
            continue

        position = next_position

def path_contains_loop(path: Iterable[Tuple[Position, Direction]]) -> bool:
    visited: set[Tuple[Position, Direction]] = set()

    for path_item in path:
        if path_item in visited:
            return True
        
        visited.add(path_item)

    return False

def parse_grid(input: str) -> Grid:
    grid: Grid = {}

    for y, line in enumerate(input.splitlines()):
        for x, char in enumerate(line):
            position = Position(x, y)
            grid[position] = char

    return grid

def find_start_position(grid: Grid) -> Position:
    for position, char in grid.items():
        if char == '^':
            return position
    
    raise Exception("starting position not found")

if __name__ == '__main__':
    main()