use advent_of_code_template::{generate_rules_map, parse_rules_and_updates, valid_update_order};

fn main() {
    let input = include_str!("input.txt");
    let (rules, updates) = parse_rules_and_updates(input);
    let rule_map = generate_rules_map(rules);

    let sum_of_middle_page_of_valid_reports: u32 = updates
        .iter()
        .filter(|update| valid_update_order(update, &rule_map))
        .map(|update| update[update.len() / 2] as u32)
        .sum();

    println!("{}", sum_of_middle_page_of_valid_reports);
}
