use advent_of_code_template::{parse_grid, XmasIter};

fn main() {
    let input = include_str!("input.txt");
    let grid = parse_grid(input);

    let count = grid
        .indices_row_major()
        .flat_map(|(row, col)| XmasIter::new((row, col), &grid).expect("position must be in grid"))
        .filter(|string| &string[..] == "XMAS")
        .count();

    println!("{count}")
}
