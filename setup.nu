#!/usr/bin/env nu

def main [
    year: int
] {
    for day in 1..25 {
        for part in 1..2 {
            echo $"day = ($day), part = ($part)"
            generate_project $year $day $part
        }
    }
}

def generate_project [
    year: int,
    day: int,
    part: int,
] {
    mut project_name = $"day($day)"

    if $part != 1 {
        $project_name = $"($project_name)_($part)"
    }

    echo $project_name

    cargo new --bin $project_name

    let project_name = $project_name

    do {
        cd $project_name
        cargo add aoc
        cargo add --path ../utils

        generate_main_rs $year $day $part | save -f src/main.rs
    }
}

def generate_main_rs [
    year: int,
    day: int,
    part: int,
] {
    $"use aoc::aoc;

#[aoc\(($year), ($day), ($part)\)]
fn main\(input: &str\) -> i32 {
    unimplemented!\(\)
}"
}