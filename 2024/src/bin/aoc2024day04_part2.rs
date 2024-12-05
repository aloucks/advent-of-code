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

fn is_mas(m: Option<char>, a: Option<char>, s: Option<char>) -> bool {
    match (m, a, s) {
        (Some('M'), Some('A'), Some('S')) => true,
        _ => false,
    }
}

fn check_xmas<const N: usize>(matrix: &MatrixData<N>, row: usize, col: usize) -> usize {
    let mut total = 0;

    // a b
    //  c
    // d e

    // diagonal-down
    let a = matrix
        .get(row + 0)
        .and_then(|row| row.get(col + 0))
        .copied();
    let c = matrix
        .get(row + 1)
        .and_then(|row| row.get(col + 1))
        .copied();
    let e = matrix
        .get(row + 2)
        .and_then(|row| row.get(col + 2))
        .copied();

    // diagonal-up
    let d = matrix
        .get(row + 2)
        .and_then(|row| row.get(col + 0))
        .copied();
    let b = matrix
        .get(row + 0)
        .and_then(|row| row.get(col + 2))
        .copied();

    if (is_mas(a, c, e) || is_mas(e, c, a)) && (is_mas(d, c, b) || is_mas(b, c, d)) {
        total += 1;
    }

    total
}

#[test]
fn test_compute_xmas() {
    let input = include_str!("aoc2024day04_part_2_input_unit_test.txt");
    let total = compute_xmas::<10>(input);
    assert_eq!(9, total);
}
