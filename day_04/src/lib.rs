use array2d::Array2D;

pub fn parse_grid(input: &str) -> Array2D<char> {
    let vector: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Array2D::from_rows(&vector).unwrap()
}

enum Line {
    None,
    DiagUpLeft,
    Up,
    DiagUpRight,
    Right,
    DiagDownRight,
    Down,
    DiagDownLeft,
    Left,
}

pub struct XmasIter<'a> {
    grid: &'a Array2D<char>,
    origin: (usize, usize),
    current: Line,
}

impl<'a> XmasIter<'a> {
    pub fn new(origin: (usize, usize), grid: &'a Array2D<char>) -> Option<Self> {
        if origin.0 >= grid.row_len() || origin.1 >= grid.column_len() {
            return None;
        }

        Some(XmasIter {
            grid,
            origin,
            current: Line::None,
        })
    }

    fn diag_up_left(&self) -> String {
        let mut string = self.grid[(self.origin.0, self.origin.1)].to_string();

        for i in 1..=3 {
            let Some(row) = self.origin.0.checked_sub(i) else {
                continue;
            };

            let Some(col) = self.origin.1.checked_sub(i) else {
                continue;
            };

            if let Some(char) = self.grid.get(row, col) {
                string.push(*char);
            }
        }

        string
    }

    fn up(&self) -> String {
        let mut string = self.grid[(self.origin.0, self.origin.1)].to_string();

        for i in 1..=3 {
            let Some(row) = self.origin.0.checked_sub(i) else {
                continue;
            };

            if let Some(char) = self.grid.get(row, self.origin.1) {
                string.push(*char);
            }
        }

        string
    }

    fn diag_up_right(&self) -> String {
        let mut string = self.grid[(self.origin.0, self.origin.1)].to_string();

        for i in 1..=3 {
            let Some(row) = self.origin.0.checked_sub(i) else {
                continue;
            };
            let col = self.origin.1 + i;

            if let Some(char) = self.grid.get(row, col) {
                string.push(*char);
            }
        }

        string
    }

    fn right(&self) -> String {
        let mut string = self.grid[(self.origin.0, self.origin.1)].to_string();

        for i in 1..=3 {
            let col = self.origin.1 + i;

            if let Some(char) = self.grid.get(self.origin.0, col) {
                string.push(*char);
            }
        }

        string
    }

    fn diag_down_right(&self) -> String {
        let mut string = self.grid[(self.origin.0, self.origin.1)].to_string();

        for i in 1..=3 {
            let row = self.origin.0 + i;
            let col = self.origin.1 + i;

            if let Some(char) = self.grid.get(row, col) {
                string.push(*char);
            }
        }

        string
    }

    fn down(&self) -> String {
        let mut string = self.grid[(self.origin.0, self.origin.1)].to_string();

        for i in 1..=3 {
            let row = self.origin.0 + i;

            if let Some(char) = self.grid.get(row, self.origin.1) {
                string.push(*char);
            }
        }

        string
    }

    fn diag_down_left(&self) -> String {
        let mut string = self.grid[(self.origin.0, self.origin.1)].to_string();

        for i in 1..=3 {
            let row = self.origin.0 + i;

            let Some(col) = self.origin.1.checked_sub(i) else {
                continue;
            };

            if let Some(char) = self.grid.get(row, col) {
                string.push(*char);
            }
        }

        string
    }

    fn left(&self) -> String {
        let mut string = self.grid[(self.origin.0, self.origin.1)].to_string();

        for i in 1..=3 {
            let Some(col) = self.origin.1.checked_sub(i) else {
                continue;
            };

            if let Some(char) = self.grid.get(self.origin.0, col) {
                string.push(*char);
            }
        }

        string
    }
}

impl<'a> Iterator for XmasIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Line::None => {
                self.current = Line::DiagUpLeft;
                Some(self.diag_up_left())
            }
            Line::DiagUpLeft => {
                self.current = Line::Up;
                Some(self.up())
            }
            Line::Up => {
                self.current = Line::DiagUpRight;
                Some(self.diag_up_right())
            }
            Line::DiagUpRight => {
                self.current = Line::Right;
                Some(self.right())
            }
            Line::Right => {
                self.current = Line::DiagDownRight;
                Some(self.diag_down_right())
            }
            Line::DiagDownRight => {
                self.current = Line::Down;
                Some(self.down())
            }
            Line::Down => {
                self.current = Line::DiagDownLeft;
                Some(self.diag_down_left())
            }
            Line::DiagDownLeft => {
                self.current = Line::Left;
                Some(self.left())
            }
            Line::Left => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        let grid = parse_grid(input);
        debug_assert_eq!(grid.get(0, 0), Some(&'.'));
        debug_assert_eq!(grid.get(0, 2), Some(&'X'));
        debug_assert_eq!(grid.get(1, 0), Some(&'.'));
        debug_assert_eq!(grid.get(1, 1), Some(&'S'));
    }

    #[test]
    fn xmas_iter() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        let grid = parse_grid(input);
        let xmas_iter = XmasIter::new((1, 4), &grid);

        let strings: Vec<String> = xmas_iter.unwrap().collect();

        debug_assert_eq!(
            strings,
            vec![
                "X.",   // diag up left
                "X.",   // up
                "X.",   // diag up right
                "X.",   // right
                "X.",   // diag down right
                "XA..", // down
                "X.AX", // diag down left
                "XMAS"  // left
            ]
        )
    }
}
