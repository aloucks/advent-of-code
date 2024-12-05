pub type MatrixData<const N: usize> = [[char; N]; N];

fn main() {
    let input = include_str!("aoc2024day04_input.txt");
    let total = compute_xmas::<140>(input);
    println!("total: {total}");
}

fn compute_xmas<const N: usize>(input: &str) -> usize {
    let mut matrix: MatrixData<140> = [[0 as char; 140]; 140];

    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().count();

    assert_eq!(N, rows);
    assert_eq!(N, cols);

    let mut row: usize = 0;
    for line in input.lines() {
        let mut col: usize = 0;
        for ch in line.chars() {
            matrix[row][col] = ch;
            col += 1;
        }
        row += 1;
    }

    let mut total = 0;

    for row in 0..rows {
        for col in 0..cols {
            total += check_xmas(&matrix, row, col);
        }
    }

    total
}

fn is_xmas(x: Option<char>, m: Option<char>, a: Option<char>, s: Option<char>) -> bool {
    match (x, m, a, s) {
        (Some('X'), Some('M'), Some('A'), Some('S')) => true,
        _ => false,
    }
}

fn check_xmas<const N: usize>(matrix: &MatrixData<N>, row: usize, col: usize) -> usize {
    let mut total = 0;

    // horizontal
    let x = matrix.get(row).and_then(|row| row.get(col + 0)).copied();
    let m = matrix.get(row).and_then(|row| row.get(col + 1)).copied();
    let a = matrix.get(row).and_then(|row| row.get(col + 2)).copied();
    let s = matrix.get(row).and_then(|row| row.get(col + 3)).copied();
    total += if is_xmas(x, m, a, s) { 1 } else { 0 };

    // horizontal-reverse
    total += if is_xmas(s, a, m, x) { 1 } else { 0 };

    // vertical
    let x = matrix.get(row + 0).and_then(|row| row.get(col)).copied();
    let m = matrix.get(row + 1).and_then(|row| row.get(col)).copied();
    let a = matrix.get(row + 2).and_then(|row| row.get(col)).copied();
    let s = matrix.get(row + 3).and_then(|row| row.get(col)).copied();
    total += if is_xmas(x, m, a, s) { 1 } else { 0 };

    // vertical-reverse
    total += if is_xmas(s, a, m, x) { 1 } else { 0 };

    // diagonal-down
    let x = matrix
        .get(row + 0)
        .and_then(|row| row.get(col + 0))
        .copied();
    let m = matrix
        .get(row + 1)
        .and_then(|row| row.get(col + 1))
        .copied();
    let a = matrix
        .get(row + 2)
        .and_then(|row| row.get(col + 2))
        .copied();
    let s = matrix
        .get(row + 3)
        .and_then(|row| row.get(col + 3))
        .copied();
    total += if is_xmas(x, m, a, s) { 1 } else { 0 };

    // diagonal-down-reverse
    total += if is_xmas(s, a, m, x) { 1 } else { 0 };

    // diagonal-up
    let x = matrix
        .get(row + 3)
        .and_then(|row| row.get(col + 0))
        .copied();
    let m = matrix
        .get(row + 2)
        .and_then(|row| row.get(col + 1))
        .copied();
    let a = matrix
        .get(row + 1)
        .and_then(|row| row.get(col + 2))
        .copied();
    let s = matrix
        .get(row + 0)
        .and_then(|row| row.get(col + 3))
        .copied();
    total += if is_xmas(x, m, a, s) { 1 } else { 0 };

    // diagonal-up-reverse
    total += if is_xmas(s, a, m, x) { 1 } else { 0 };

    total
}

#[test]
fn test_compute_xmas() {
    let input = include_str!("aoc2024day04_input_unit_test.txt");
    let total = compute_xmas::<10>(input);
    assert_eq!(18, total);
}
