use std::fmt::Display;

fn main() {
    let input = include_str!("aoc2025day04_input.txt");
    let mut grid = Grid::load(input);
    grid.mark_accessible();
    let accessible = grid.count_accessible();
    println!("accessible: {accessible}");
}

pub const TEST_CASE: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

pub const TEST_CASE_SOLUTION: &str = r#"
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
"#;

#[derive(Debug, PartialEq, Eq)]
struct GridCell {
    empty: bool,
    marked_accessible: bool,
}

impl Display for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.marked_accessible, self.empty) {
            (true, _) => write!(f, "x"),
            (false, true) => write!(f, "."),
            (false, false) => write!(f, "@"),
        }
    }
}

impl GridCell {
    fn from_char(ch: char) -> GridCell {
        match ch {
            '@' => GridCell {
                empty: false,
                marked_accessible: false,
            },
            'x' => GridCell {
                empty: false,
                marked_accessible: true,
            },
            '.' => GridCell {
                empty: true,
                marked_accessible: false,
            },
            _ => panic!("invalid grid character: {}", ch),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    values: Vec<Vec<GridCell>>,
}

impl Grid {
    fn load(string: &str) -> Grid {
        let mut values = Vec::new();
        for line in string.lines() {
            let mut row = Vec::new();
            if line.trim().is_empty() {
                continue;
            }
            for ch in line.chars() {
                row.push(GridCell::from_char(ch));
            }
            values.push(row);
        }
        Grid { values }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.values.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn mark_accessible(&mut self) {
        let row_count = self.values.len();
        for j in 0..row_count {
            let col_count = self.values[j].len();
            for i in 0..col_count {
                let mut non_empty_adjacent_count = 0;
                if self.values[j][i].empty {
                    continue;
                }
                for (adj_i, adj_j) in self.iter_adjacent_cell_indices(i, j) {
                    if !self.values[adj_j][adj_i].empty {
                        non_empty_adjacent_count += 1;
                    }
                }
                if non_empty_adjacent_count < 4 {
                    self.values[j][i].marked_accessible = true;
                }
            }
        }
    }

    pub fn count_accessible(&self) -> usize {
        self.values
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.marked_accessible)
            .count()
    }

    fn iter_adjacent_cell_indices(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut adjacent_cell_indices = Vec::new();
        let values_len = self.values.len();
        let (x, y) = (x as isize, y as isize);
        for x_offset in [-1, 0, 1_isize] {
            for y_offset in [-1, 0, 1_isize] {
                // if let Some(adj_x) = x.checked_add(x_offset) && let Some(adj_y) = y.checked_add(y_offset) {

                // }

                let (i, j) = (x + x_offset, y + y_offset);

                // skip the focus cell
                if i == x && j == y {
                    continue;
                }

                // skip out of bounds: y
                if j < 0 || j >= values_len as isize {
                    continue;
                }

                let row = &self.values[j as usize];

                // skip out of bounds: x
                if i < 0 || i >= row.len() as isize {
                    continue;
                }

                adjacent_cell_indices.push((i as usize, j as usize));
            }
        }

        adjacent_cell_indices.into_iter()
    }
}

#[test]
fn test_iter_adjacent_cell_indices() {
    let grid = Grid::load(TEST_CASE);
    let mut a = grid.iter_adjacent_cell_indices(0, 0);
    assert_eq!(Some((0, 1)), a.next());
    assert_eq!(Some((1, 0)), a.next());
    assert_eq!(Some((1, 1)), a.next());
    assert_eq!(None, a.next());

    // for val in grid.iter_adjacent_cell_indices(1, 1).enumerate() {
    //     println!("{val:?}")
    // }

    let mut b = grid.iter_adjacent_cell_indices(1, 1);
    assert_eq!(Some((0, 0)), b.next());
    assert_eq!(Some((0, 1)), b.next());
    assert_eq!(Some((0, 2)), b.next());
    assert_eq!(Some((1, 0)), b.next());
    assert_eq!(Some((1, 2)), b.next());
    assert_eq!(Some((2, 0)), b.next());
    assert_eq!(Some((2, 1)), b.next());
    assert_eq!(Some((2, 2)), b.next());
    assert_eq!(None, b.next());
}

#[test]
fn test_count_accessible() {
    let solution = Grid::load(TEST_CASE_SOLUTION);
    assert_eq!(13, solution.count_accessible());
}

#[test]
fn test_mark_accessible() {
    let mut grid = Grid::load(TEST_CASE);
    println!("{}", grid);
    grid.mark_accessible();
    println!("{}", grid);
    let solution = Grid::load(TEST_CASE_SOLUTION);
    println!("{}", solution);
    assert_eq!(solution, grid);
}
