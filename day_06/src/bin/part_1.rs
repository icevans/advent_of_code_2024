use advent_of_code_template::Guard;
use array2d::Array2D;

fn main() {
    let input = include_str!("input.txt");

    let vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid = Array2D::from_rows(&vec).unwrap();

    let mut guard = Guard::new((52, 48), &grid);

    while guard.position.is_some() {
        guard.walk_forward();
        println!("{:?}", guard.position);
    }

    println!("{}", guard.num_visited());
}
