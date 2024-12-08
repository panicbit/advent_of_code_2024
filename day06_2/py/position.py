from dataclasses import dataclass
from direction import Direction
from copy import copy

@dataclass(frozen=True)
class Position:
    x: int
    y: int

    def __add__(self, direction: Direction) -> "Position":
        x = self.x
        y = self.y

        match direction:
            case Direction.Up: y -= 1
            case Direction.Right: x += 1
            case Direction.Down: y += 1
            case Direction.Left: x -= 1

        return Position(x, y)