use std::ops;

use aoc::aoc;
use regex::Regex;

#[aoc(2024, 14, 1)]
fn main(input: &str) -> usize {
    let width = 101;
    let height = 103;
    let num_iterations = 100;
    let dimensions = Vec2::new(width, height);
    let mut robots = input.lines().map(Robot::parse).collect::<Vec<_>>();

    for robot in &mut robots {
        robot.position =
            dimensions * num_iterations + robot.position + robot.velocity * num_iterations;
        robot.position = robot.position % dimensions;
    }

    let center = dimensions / 2;
    let top_left = robots
        .iter()
        .filter(|robot| robot.position.x < center.x && robot.position.y < center.y)
        .count();
    let top_right = robots
        .iter()
        .filter(|robot| robot.position.x > center.x && robot.position.y < center.y)
        .count();
    let bottom_left = robots
        .iter()
        .filter(|robot| robot.position.x < center.x && robot.position.y > center.y)
        .count();
    let bottom_right = robots
        .iter()
        .filter(|robot| robot.position.x > center.x && robot.position.y > center.y)
        .count();

    top_left * top_right * bottom_left * bottom_right
}

struct Robot {
    position: Vec2,
    velocity: Vec2,
}

impl Robot {
    fn parse(robot: &str) -> Self {
        let re =
            Regex::new(r"^p=(?P<px>-?\d+),(?P<py>-?\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)$").unwrap();

        let caps = re.captures(robot).unwrap();

        Self {
            position: Vec2 {
                x: caps["px"].parse().unwrap(),
                y: caps["py"].parse().unwrap(),
            },
            velocity: Vec2 {
                x: caps["vx"].parse().unwrap(),
                y: caps["vy"].parse().unwrap(),
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn splat(n: isize) -> Self {
        Self::new(n, n)
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Div<isize> for Vec2 {
    type Output = Self;

    fn div(self, rhs: isize) -> Self::Output {
        self / Vec2::splat(rhs)
    }
}

impl ops::Rem for Vec2 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl ops::Rem<isize> for Vec2 {
    type Output = Self;

    fn rem(self, rhs: isize) -> Self::Output {
        self % Vec2::splat(rhs)
    }
}

impl ops::Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::Mul<isize> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: isize) -> Self::Output {
        self * Vec2::splat(rhs)
    }
}
