use aoc::aoc;
use regex::Regex;

#[aoc(2024, 13, 1)]
fn main(input: &str) -> u64 {
    let machines = input.split("\n\n").map(parse_machine).collect::<Vec<_>>();
    let tokens = machines.iter().filter_map(|machine| machine.tokens()).sum();

    tokens
}

fn parse_machine(machine: &str) -> Machine {
    let mut lines = machine.lines();

    Machine {
        a: parse_button(lines.next().unwrap()),
        b: parse_button(lines.next().unwrap()),
        prize: parse_prize(lines.next().unwrap()),
    }
}

fn parse_button(button: &str) -> Vec2 {
    let re = Regex::new(r"^Button [AB]: X\+(?P<x>\d+), Y\+(?P<y>\d+)$").unwrap();
    let caps = re.captures(button).unwrap();

    Vec2 {
        x: caps["x"].parse().unwrap(),
        y: caps["y"].parse().unwrap(),
    }
}

fn parse_prize(price: &str) -> Vec2 {
    let re = Regex::new(r"^Prize: X=(?P<x>\d+), Y=(?P<y>\d+)$").unwrap();
    let caps = re.captures(price).unwrap();

    Vec2 {
        x: caps["x"].parse().unwrap(),
        y: caps["y"].parse().unwrap(),
    }
}

#[derive(Debug)]
struct Machine {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}

impl Machine {
    fn tokens(&self) -> Option<u64> {
        let mut min_tokens = None;

        for a in 0..=100 {
            for b in 0..=100 {
                let x = a * self.a.x + b * self.b.x;
                let y = a * self.a.y + b * self.b.y;
                let tokens = a * 3 + 1 * b;

                if x != self.prize.x || y != self.prize.y {
                    continue;
                }

                min_tokens = Some((*min_tokens.get_or_insert(tokens)).min(tokens));
            }
        }

        min_tokens
    }
}

#[derive(Debug)]
struct Vec2 {
    x: u64,
    y: u64,
}
