use array2d::Array2D;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Guard<'a> {
    pub position: Option<(usize, usize)>,
    direction: Direction,
    map: &'a Array2D<char>,
    visited: HashSet<(usize, usize)>,
}

impl<'a> Guard<'a> {
    pub fn new(position: (usize, usize), map: &'a Array2D<char>) -> Self {
        Guard {
            position: Some(position),
            map,
            direction: Direction::Up,
            visited: HashSet::new(),
        }
    }

    pub fn num_visited(&self) -> usize {
        self.visited.len()
    }

    pub fn walk_forward(&mut self) {
        let Some(current_position) = self.position else {
            self.position = None;
            return;
        };

        self.visited.insert(current_position);

        let next_position: (usize, usize);
        match self.direction {
            Direction::Up => {
                if let Some(new_row) = current_position.0.checked_sub(1) {
                    next_position = (new_row, current_position.1);
                } else {
                    self.position = None;
                    return;
                }
            }
            Direction::Right => next_position = (current_position.0, current_position.1 + 1),
            Direction::Down => next_position = (current_position.0 + 1, current_position.1),
            Direction::Left => {
                if let Some(new_col) = current_position.1.checked_sub(1) {
                    next_position = (current_position.0, new_col);
                } else {
                    self.position = None;
                    return;
                }
            }
        }

        let Some(square) = self.map.get(next_position.0, next_position.1) else {
            self.position = None;
            return;
        };

        if square == &'#' {
            self.direction = match self.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        } else {
            self.position = Some(next_position);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn it_works() {
        let input = "....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...";

        let vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let grid = Array2D::from_rows(&vec).unwrap();

        let mut guard = Guard::new((1, 4), &grid);

        while guard.position.is_some() {
            guard.walk_forward();
            println!("{:?}", guard.position);
        }

        debug_assert_eq!(guard.num_visited(), 41);
    }
}
