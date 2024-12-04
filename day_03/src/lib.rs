use regex::Regex;

pub fn mul_parser(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| {
            let x = c.get(1).expect("regexp has 2 capture groups").as_str();
            let y = c.get(2).expect("regexp has 2 capture groups").as_str();

            (
                x.parse().expect("capture group contained only digits"),
                y.parse().expect("capture group contained only digits"),
            )
        })
        .collect()
}

pub fn disablable_mul_parser(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut mul_operations = vec![];
    let mut enabled = true;

    for capture in re.captures_iter(input) {
        let the_match = capture.get(0).unwrap().as_str();
        match the_match {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if !enabled {
                    continue;
                }

                let x = capture
                    .get(1)
                    .expect("missing 1 but regexp has 2 capture groups")
                    .as_str();
                let y = capture
                    .get(2)
                    .expect("missing 2 but regexp has 2 capture groups")
                    .as_str();

                mul_operations.push((
                    x.parse().expect("capture group contained only digits"),
                    y.parse().expect("capture group contained only digits"),
                ));
            }
        }
    }

    mul_operations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let parsed = mul_parser(sample);
        debug_assert_eq!(parsed, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn disableable() {
        let sample = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let parsed = disablable_mul_parser(sample);
        debug_assert_eq!(parsed, vec![(2, 4), (8, 5)]);
    }
}
