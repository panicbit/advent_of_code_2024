use std::iter;

use aoc::aoc;
use itertools::Itertools;
use utils::{tuple_split, StrExt};

#[aoc(2024, 7, 1)]
fn main(input: &str) -> i64 {
    let equations = parse_equations(input);

    equations
        .iter()
        .filter(|equation| equation.can_be_true())
        .map(|equation| equation.left)
        .sum()
}

fn parse_equations(equations: &str) -> Vec<Equation> {
    equations.lines().map(Equation::parse).collect()
}

fn possible_operations(len: usize) -> impl Iterator<Item = Vec<char>> {
    iter::repeat_n(['+', '*'], len).multi_cartesian_product()
}

#[derive(Debug)]
struct Equation {
    left: i64,
    right: Vec<i64>,
}

impl Equation {
    fn parse(equation: &str) -> Self {
        let (left, right) = tuple_split(equation, ": ");
        let left = left.i64();
        let right = right.split(" ").map(<_>::i64).collect();

        Self { left, right }
    }

    fn can_be_true(&self) -> bool {
        for operations in possible_operations(self.right.len() - 1) {
            let mut right = self.right.iter().copied();
            let first = right.next().unwrap();
            let value = right
                .zip(operations)
                .fold(first, |acc, (value, op)| match op {
                    '+' => acc + value,
                    '*' => acc * value,
                    _ => panic!("unknown op: {op}"),
                });

            if value == self.left {
                return true;
            }
        }

        false
    }
}
