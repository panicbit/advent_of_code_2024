use aoc::aoc;

#[aoc(2024, 3, 2)]
fn main(input: &str) -> i32 {
    let instructions = parse_instructions(input);

    let mut sum = 0;
    let mut enabled = true;

    for instruction in &instructions {
        match instruction {
            Instruction::Mul(a, b) => {
                if enabled {
                    sum += a * b
                }
            }
            Instruction::Dont => enabled = false,
            Instruction::Do => enabled = true,
        }
    }

    sum
}

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Dont,
    Do,
}

fn parse_instructions(mut input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    while !input.is_empty() {
        if let Some(instruction) = parse_instruction(input) {
            instructions.push(instruction);
        }

        input = &input[1..];
    }

    instructions
}

fn parse_instruction(input: &str) -> Option<Instruction> {
    if input.starts_with("don't()") {
        Some(Instruction::Dont)
    } else if input.starts_with("do()") {
        Some(Instruction::Do)
    } else if input.starts_with("mul") {
        let input = input.strip_prefix("mul").unwrap();

        let (a, b) = parse_mul_args(input)?;

        Some(Instruction::Mul(a, b))
    } else {
        None
    }
}

fn parse_mul_args(input: &str) -> Option<(i32, i32)> {
    let input = input.strip_prefix("(")?;
    let (a, input) = parse_number(input)?;
    let input = input.strip_prefix(",")?;
    let (b, input) = parse_number(input)?;
    input.strip_prefix(")")?;

    Some((a, b))
}

fn parse_number(input: &str) -> Option<(i32, &str)> {
    let last_digit_index = input
        .char_indices()
        .take_while(|(_, ch)| ch.is_ascii_digit())
        .map(|(index, _)| index)
        .last()?;

    let (number, input) = input.split_at(last_digit_index + 1);
    let number = number.parse::<i32>().ok()?;

    Some((number, input))
}
