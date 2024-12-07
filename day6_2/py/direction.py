from enum import Enum

class Direction(Enum):
    Up = 1
    Right = 2
    Down = 3
    Left = 4

    def rotated_right(self) -> "Direction":
        match self:
            case Direction.Up: return Direction.Right
            case Direction.Right: return Direction.Down
            case Direction.Down: return Direction.Left
            case Direction.Left: return Direction.Up
