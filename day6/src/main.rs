use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
struct Coordinates {
    row: usize,
    col: usize,
}

type PuzzleMap = Vec<Vec<char>>;

fn main() {
    let mut map: PuzzleMap = vec![];
    let mut obstructions_placed = 0; // count of number of obstructions that would put the guard in an infinite loop

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            map.push(line.chars().collect());
        }
    }

    let mut guard_position = get_guard_position(&map);

    while guard_is_in_puzzle_map(&map, &guard_position) {
        if try_trapping_guard(&mut map.clone(), &mut guard_position.clone()) {
            obstructions_placed += 1;
        }
        guard_step(&mut map, &mut guard_position, 'X');
    }

    println!("count_guard_positions {}", count_guard_positions(&map));
    println!("obstructions_placed {}", obstructions_placed);
    
}

fn try_trapping_guard(map: &mut PuzzleMap, guard_position: &mut Coordinates) -> bool {
    // set an obstacle in the guard's path and turn them
    match map[guard_position.row][guard_position.col] {
        '^' => {
            if guard_position.row == 0 || map[guard_position.row - 1][guard_position.col] == '#'
            || guard_position.col + 1 == map[0].len() {
                return false;
            }
            map[guard_position.row - 1][guard_position.col] = 'O';
            map[guard_position.row][guard_position.col] = 'S';
            guard_position.col += 1;
            map[guard_position.row][guard_position.col] = '>';
            
        },
        'v' => {
            if guard_position.row + 1 == map.len() 
            || map[guard_position.row + 1][guard_position.col] == '#'
            || guard_position.col == 0 {
                return false;
            }
            map[guard_position.row + 1][guard_position.col] = 'O';
            map[guard_position.row][guard_position.col] = 'S';
            guard_position.col -= 1;
            map[guard_position.row][guard_position.col] = '<';

        },
        '>' => {
            if guard_position.col + 1 == map[0].len() 
            || map[guard_position.row][guard_position.col + 1] == '#'
            || guard_position.row + 1 == map.len() {
                return false;
            }
            map[guard_position.row][guard_position.col + 1] = 'O';
            map[guard_position.row][guard_position.col] = 'S';
            guard_position.row += 1;
            map[guard_position.row][guard_position.col] = 'v';


        },
        '<' => {
            if guard_position.col == 0 
            || map[guard_position.row][guard_position.col - 1] == '#'
            || guard_position.row == 0 {
                return false;
            }
            map[guard_position.row][guard_position.col - 1] = 'O';
            map[guard_position.row][guard_position.col] = 'S';
            guard_position.row -= 1;
            map[guard_position.row][guard_position.col] = '^';

        },
        _ => panic!(),
    }
    
    while guard_is_in_puzzle_map(&map, &guard_position) {
        if is_guard_at_start_of_loop(&map, &guard_position) {
            return true;
        }
        guard_step(map, guard_position, 'Y');
    }
    return false;
}

fn is_guard_at_start_of_loop(map: &PuzzleMap, guard_position: &Coordinates) -> bool {
    match map[guard_position.row][guard_position.col] {
        '^' => {
            if guard_position.row == 0 {
                return false;
            } else if map[guard_position.row - 1][guard_position.col] == 'O'
            || map[guard_position.row - 1][guard_position.col] == 'S' {
                return true;
            }
        },
        'v' => {
            if guard_position.row + 1 == map.len() {
                return false;
            } else if map[guard_position.row + 1][guard_position.col] == 'O'
            || map[guard_position.row + 1][guard_position.col] == 'S' {
                return true;
            }

        },
        '>' => {
            if guard_position.col + 1 == map[0].len() {
                return false;
            } else if map[guard_position.row][guard_position.col + 1] == 'O'
            || map[guard_position.row][guard_position.col + 1] == 'S' {
                return true;
            }
        },
        '<' => {
            if guard_position.col == 0 {
                return false;
            } else if map[guard_position.row][guard_position.col - 1] == 'O'
            || map[guard_position.row][guard_position.col - 1] == 'S' {
                return true;
            }
        },
        _ => panic!(),
    }

    

    return false;
}

fn count_guard_positions(map: &PuzzleMap) -> u32 {
    let mut count: u32 = 0;
    for row in map {
        for col in row {
            if *col == 'X' {
                count += 1;
            }
        }
    }
    return count;
}

fn guard_is_in_puzzle_map(map: &PuzzleMap, guard_position: &Coordinates) -> bool {
    return guard_position.row < map.len() && guard_position.col < map[0].len();
}

fn guard_step(map: &mut PuzzleMap, guard_position: &mut Coordinates, guard_position_marker: char) {

    let mut new_guard_position = Coordinates {row: guard_position.row, col: guard_position.col};
    
    match map[guard_position.row][guard_position.col] {
        '^' => {
            // try moving up
            if new_guard_position.row == 0 {
                new_guard_position.row = usize::MAX;
            }
            new_guard_position.row -= 1;
        },
        'v' => {
            // try moving down
            new_guard_position.row += 1;
        },
        '>' => {
            // try moving right
            new_guard_position.col += 1;
        },
        '<' => {
            // try moving left
            if new_guard_position.col == 0 {
                new_guard_position.col = usize::MAX;
            }
            new_guard_position.col -= 1;
        },
        _ => panic!(),
    }

    if guard_is_in_puzzle_map(&map, &new_guard_position) {
        // check for an obstacle
        if map[new_guard_position.row][new_guard_position.col] == '#' {
            // turn right
            match map[guard_position.row][guard_position.col] {
                '^' => {
                    map[guard_position.row][guard_position.col] = '>';
                },
                'v' => {
                    map[guard_position.row][guard_position.col] = '<';
                },
                '>' => {
                    map[guard_position.row][guard_position.col] = 'v';
                },
                '<' => {
                    map[guard_position.row][guard_position.col] = '^';
                },
                _ => panic!(),
            }
        } else {
            // no obstacle, continue forward
            map[new_guard_position.row][new_guard_position.col] = map[guard_position.row][guard_position.col];
            map[guard_position.row][guard_position.col] = guard_position_marker; // mark where the guard has been
            guard_position.row = new_guard_position.row;
            guard_position.col = new_guard_position.col;
        }
    } else {
        // moved out of the area
        map[guard_position.row][guard_position.col] = guard_position_marker;
        guard_position.row = new_guard_position.row;
        guard_position.col = new_guard_position.col;
    }
}

fn get_guard_position(map: &PuzzleMap) -> Coordinates {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                '^' | 'v' | '>' | '<' => return Coordinates { row: row, col: col},
                _ => continue,
            }
        }
    }
    panic!();
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

