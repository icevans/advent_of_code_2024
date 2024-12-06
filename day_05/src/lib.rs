use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn parse_rules_and_updates(input: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let rule_re = Regex::new(r"(?m)^(\d+)\|(\d+)$").unwrap();

    let input_chunks: Vec<&str> = input.split("\n\n").collect();
    let raw_rules = input_chunks[0];
    let raw_updates = input_chunks[1];

    let parsed_rules: Vec<(u8, u8)> = rule_re
        .captures_iter(raw_rules)
        .map(|c| {
            let x: u8 = c
                .get(1)
                .expect("two capture groups")
                .as_str()
                .parse()
                .expect("digits only");
            let y: u8 = c
                .get(2)
                .expect("two capture groups")
                .as_str()
                .parse()
                .expect("digits only");
            (x, y)
        })
        .collect();

    let parsed_updates: Vec<Vec<u8>> = raw_updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|c| c.parse().expect("digits only"))
                .collect()
        })
        .collect();

    (parsed_rules, parsed_updates)
}

pub fn generate_rules_map(rules: Vec<(u8, u8)>) -> HashMap<u8, Vec<u8>> {
    let mut rule_map = HashMap::new();

    for rule in rules {
        rule_map
            .entry(rule.0)
            .and_modify(|befores: &mut Vec<u8>| befores.push(rule.1))
            .or_insert(vec![rule.1]);
    }

    rule_map
}

pub fn valid_update_order(update: &Vec<u8>, rule_map: &HashMap<u8, Vec<u8>>) -> bool {
    let mut seen_pages = HashSet::new();

    for page in update {
        seen_pages.insert(page);

        let Some(pages_that_must_be_after_current) = rule_map.get(&page) else {
            continue;
        };

        if pages_that_must_be_after_current
            .iter()
            .any(|page| seen_pages.contains(page))
        {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
}
