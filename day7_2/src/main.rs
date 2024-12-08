use std::iter;

use aoc::aoc;
use itertools::Itertools;
use utils::{tuple_split, StrExt};

#[aoc(2024, 7, 2)]
fn main(input: &str) -> i64 {
    let equations = parse_equations(input);

    equations
        .iter()
        .filter(|equation| equation.can_be_true())
        .map(|equation| equation.test_value)
        .sum()
}

fn parse_equations(equations: &str) -> Vec<Equation> {
    equations.lines().map(Equation::parse).collect()
}

fn possible_operations(len: usize) -> impl Iterator<Item = Vec<&'static str>> {
    iter::repeat_n(["+", "*", "||"], len).multi_cartesian_product()
}

#[derive(Debug)]
struct Equation {
    test_value: i64,
    values: Vec<i64>,
}

impl Equation {
    fn parse(equation: &str) -> Self {
        let (test_value, values) = tuple_split(equation, ": ");
        let test_value = test_value.i64();
        let values = values.split(" ").map(<_>::i64).collect();

        Self { test_value, values }
    }

    fn can_be_true(&self) -> bool {
        for operations in possible_operations(self.values.len() - 1) {
            let mut values = self.values.iter().copied();
            let first = values.next().unwrap();
            let result = values
                .zip(operations)
                .fold(first, |acc, (value, op)| match op {
                    "+" => acc + value,
                    "*" => acc * value,
                    "||" => format!("{acc}{value}").i64(),
                    _ => panic!("unknown op: {op}"),
                });

            if result == self.test_value {
                return true;
            }
        }

        false
    }
}
