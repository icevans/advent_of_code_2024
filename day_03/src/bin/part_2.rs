use advent_of_code_template::disablable_mul_parser;

fn main() {
    let input = include_str!("input.txt");
    let mul_operations = disablable_mul_parser(input);

    let sum: u32 = mul_operations.into_iter().map(|(x, y)| x * y).sum();

    println!("{sum}");
}
