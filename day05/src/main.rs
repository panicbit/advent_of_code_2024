use aoc::aoc;
use utils::{array_split, tuple_split_parse, StrExt};

#[aoc(2024, 5, 1)]
fn main(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| update_is_valid(update, &rules))
        .map(middle_page_number)
        .sum()
}

type Rule = (i32, i32);
type Update = Vec<i32>;

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let [rules, updates] = array_split(input, "\n\n");
    let rules = parse_rules(rules);
    let updates = parse_updates(updates);

    (rules, updates)
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    rules
        .lines()
        .map(|line| tuple_split_parse(line, "|"))
        .collect::<Vec<Rule>>()
}

fn parse_updates(updates: &str) -> Vec<Update> {
    updates.lines().map(parse_update).collect::<Vec<_>>()
}

fn parse_update(update: &str) -> Update {
    update.split(",").map(|page| page.i32()).collect::<Vec<_>>()
}

fn update_is_valid(update: &Update, rules: &[Rule]) -> bool {
    for rule in rules {
        let Some((pos_a, pos_b)) = find_rule_page_positions(update, rule) else {
            continue;
        };

        if pos_a > pos_b {
            return false;
        }
    }

    true
}

fn find_rule_page_positions(update: &Update, (page_a, page_b): &Rule) -> Option<(usize, usize)> {
    let pos_a = update.iter().position(|page| page == page_a)?;
    let pos_b = update.iter().position(|page| page == page_b)?;

    Some((pos_a, pos_b))
}

fn middle_page_number(update: &Update) -> i32 {
    update[update.len() / 2]
}
