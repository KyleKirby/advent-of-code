use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() {
    let mut all_lines: Vec<Vec<char>> = vec![];

    let xmas: Vec<char> = vec!['X', 'M', 'A', 'S'];

    let mut xmas_count = 0;

    let mut x_mas_count = 0;

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            all_lines.push(line.chars().collect());
        }
    }

    for i in 0..all_lines.len() {
        for j in 0..all_lines[i].len() {
            // find an X
            if all_lines[i][j] == 'X' {
                // find XMAS

                if find_xmas_right(&all_lines, i, j, &xmas) {
                    xmas_count += 1;
                }

                if find_xmas_left(&all_lines, i, j, &xmas) {
                    xmas_count += 1;
                }

                if find_xmas_up(&all_lines, i, j, &xmas) {
                    xmas_count += 1;
                }

                if find_xmas_down(&all_lines, i, j, &xmas) {
                    xmas_count += 1;
                }

                if find_xmas_down_diagonal_right(&all_lines, i, j, &xmas) {
                    xmas_count += 1;
                }

                if find_xmas_down_diagonal_left(&all_lines, i, j, &xmas) {
                    xmas_count += 1;
                }

                if find_xmas_up_diagonal_right(&all_lines, i, j, &xmas) {
                    xmas_count += 1;
                }

                if find_xmas_up_diagonal_left(&all_lines, i, j, &xmas) {
                    xmas_count += 1;
                }
            } else if all_lines[i][j] == 'A' {
                // X-MAS must have an 'A' in the middle so start with that and then search for 'M' and 'S' on the diagonal
                if find_x_mas(&all_lines, i, j, &xmas) {
                    x_mas_count += 1;
                }
            }
        }
    }
    println!("{:?}", all_lines);

    println!("xmas_count {} x_mas_count {}", xmas_count, x_mas_count);
}

fn find_xmas_right(all_lines: &Vec<Vec<char>>, row: usize, col: usize, xmas: &Vec<char>) -> bool {
    if col + 3 >= all_lines[row].len() {
        return false;
    }

    // look right
    for xmas_index in 1..4 {
        if all_lines[row][col + xmas_index] != xmas[xmas_index] {
            return false;
        }
    }
    return true;
}

fn find_xmas_left(all_lines: &Vec<Vec<char>>, row: usize, col: usize, xmas: &Vec<char>) -> bool {
    if col < 3 {
        return false;
    }
    // look left
    for xmas_index in 1..4 {
        if all_lines[row][col - xmas_index] != xmas[xmas_index] {
            return false;
        }
    }
    return true;
}

fn find_xmas_up(all_lines: &Vec<Vec<char>>, row: usize, col: usize, xmas: &Vec<char>) -> bool {
    if row < 3 {
        // look up
        return false;
    }

    for xmas_index in 1..4 {
        if all_lines[row - xmas_index][col] != xmas[xmas_index] {
            return false;
        }
    }
    return true;
}

fn find_xmas_down(all_lines: &Vec<Vec<char>>, row: usize, col: usize, xmas: &Vec<char>) -> bool {
    if row + 3 >= all_lines.len() {
        return false;
    }

    // look down
    for xmas_index in 1..4 {
        if all_lines[row + xmas_index][col] != xmas[xmas_index] {
            return false;
        }
    }
    return true;
}

fn find_xmas_down_diagonal_right(all_lines: &Vec<Vec<char>>, row: usize, col: usize, xmas: &Vec<char>) -> bool {
    if row + 3 >= all_lines.len() || col + 3 >= all_lines[row].len() {
        return false;
    }

    // look down diagonal right
    for xmas_index in 1..4 {
        if all_lines[row + xmas_index][col + xmas_index] != xmas[xmas_index] {
            return false;
        }
    }
    return true;
}

fn find_xmas_down_diagonal_left(all_lines: &Vec<Vec<char>>, row: usize, col: usize, xmas: &Vec<char>) -> bool {
    if row + 3 >= all_lines.len() || col < 3 {
        return false;
    }

    // look down diagonal left
    for xmas_index in 1..4 {
        if all_lines[row + xmas_index][col - xmas_index] != xmas[xmas_index] {
            return false;
        }
    }
    return true;
}

fn find_xmas_up_diagonal_right(all_lines: &Vec<Vec<char>>, row: usize, col: usize, xmas: &Vec<char>) -> bool {
    if row < 3 || col + 3 >= all_lines[row].len() {
        return false;
    }

    // look down diagonal right
    for xmas_index in 1..4 {
        if all_lines[row - xmas_index][col + xmas_index] != xmas[xmas_index] {
            return false;
        }
    }
    return true;
}

fn find_xmas_up_diagonal_left(all_lines: &Vec<Vec<char>>, row: usize, col: usize, xmas: &Vec<char>) -> bool {
    if row < 3 || col < 3 {
        return false;
    }

    // look down diagonal left
    for xmas_index in 1..4 {
        if all_lines[row - xmas_index][col - xmas_index] != xmas[xmas_index] {
            return false;
        }
    }
    return true;
}

fn find_x_mas(all_lines: &Vec<Vec<char>>, row: usize, col: usize, xmas: &Vec<char>) -> bool {
    if col < 1 || row < 1 || col + 1 >= all_lines[row].len() || row + 1 >= all_lines.len() {
        return false;
    }

    // check left diagonal
    match all_lines[row - 1][col - 1] {
        'M' => {
            if all_lines[row + 1][col + 1] != 'S' {
                return false;
            }
        },
        'S' => {
            if all_lines[row + 1][col + 1] != 'M' {
                return false;
            }
        },
        _ => return false,
    }

    // check right diagonal
    match all_lines[row - 1][col + 1] {
        'M' => {
            if all_lines[row + 1][col - 1] != 'S' {
                return false;
            }
        },
        'S' => {
            if all_lines[row + 1][col - 1] != 'M' {
                return false;
            }
        },
        _ => return false,
    }

    return true;
}




// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}