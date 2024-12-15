use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
//use std::{thread, time};
use std::env;


#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Coordinates {
    row: usize,
    col: usize,
}

type PuzzleMap = Vec<Vec<char>>;

type PuzzleHashSet = HashSet<Coordinates>;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut map: PuzzleMap = vec![];

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            map.push(line.chars().collect());
        }
    }

    let mut guard_position = get_guard_position(&map);
    let mut obstacle_set = PuzzleHashSet::new();
    let mut guard_step_set = PuzzleHashSet::new();
    let starting_position = guard_position.clone();

    // try placing an obstacle at each position
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            let obstacle_position = Coordinates {row: row, col: col};
            if map[row][col] == '#' ||  obstacle_position == starting_position {
                // can't place an obstacle here
                continue;
            }

            try_trapping_guard(&mut map.clone(), &mut guard_position.clone(), &mut obstacle_set, &obstacle_position);
        }
    }

    while guard_is_in_puzzle_map(&map, &guard_position) {
        guard_step_set.insert(guard_position);
        guard_step(&mut map, &mut guard_position, 'X');
    }

    println!("guard positions {}", guard_step_set.len());
    println!("obstacles placed {}", obstacle_set.len());
    
}

fn print_map(map: &PuzzleMap, guard_position: &Coordinates) {
    let mut start_row = guard_position.row;
    let mut start_col = guard_position.col;
    let mut end_row = guard_position.row;
    let mut end_col = guard_position.col;

    let mut row_offset = 20;
    let mut col_offset = 20;

    if start_row >= row_offset {
        start_row -= row_offset;
    } else {
        row_offset += start_row;
        start_row = 0;
    }

    if start_col >= col_offset {
        start_col -= col_offset;
    } else {
        col_offset += start_col;
        start_col = 0;
    }

    if end_row + row_offset >= map.len() {
        end_row = map.len();
    } else {
        end_row += row_offset;
    }

    if end_col + col_offset >= map[0].len() {
        end_col = map[0].len();
    } else {
        end_col += col_offset;
    }

    // reset console
    print!("\x1B[2J\x1B[1;1H");

    for row in start_row..end_row {
        for col in start_col..end_col {
            print!("{} ", map[row][col]);
        }
        println!("");
    }
}

fn try_trapping_guard(map: &mut PuzzleMap, guard_position: &mut Coordinates, obstacle_set: &mut PuzzleHashSet, obstacle_position: &Coordinates) -> bool {
    // set an obstacle in the guard's path and see if they get in a loop

    //println!("{},{} try trapping at", guard_position.row, guard_position.col);

    let mut puzzle_set = PuzzleHashSet::new();

    map[obstacle_position.row][obstacle_position.col] = 'O';

    let mut count_down = 0;
    let mut last_marker_count = 0;
    let mut current_marker_count: usize;
    
    while guard_is_in_puzzle_map(&map, &guard_position) {

        //print_map(&map, &guard_position);
        //thread::sleep(time::Duration::from_millis(10));

        /*
        if is_guard_at_start_of_loop(&map, &guard_position) {
            return true;
        }
        */
        puzzle_set.insert(*guard_position);

        guard_step(map, guard_position, '/');

        // count the number of markers at each step and see if it is increasing
        current_marker_count = puzzle_set.len();
        if current_marker_count == last_marker_count {
            if count_down == 0 {
                /*
                * start counting down from the current count and give the guard that many steps to see if it is stuck in a loop
                */
                count_down = 4 + current_marker_count;
            } else {
                count_down -= 1;
                
                //println!("{} {} {}", current_marker_count, last_marker_count, count_down);
                if count_down == 0 {
                    // this is likely a loop
                    obstacle_set.insert(*obstacle_position);
                    //println!("=========================================================================");
                    //print_map(&map, &guard_position);
                    return true;
                }
            }
        } else {
            count_down = 0;
            last_marker_count = current_marker_count;
        }
    }
    return false;
}

fn guard_is_in_puzzle_map(map: &PuzzleMap, guard_position: &Coordinates) -> bool {
    return guard_position.row < map.len() && guard_position.col < map[0].len();
}

fn guard_step(map: &mut PuzzleMap, guard_position: &mut Coordinates, guard_position_marker: char) {

    //println!("{},{}", guard_position.row, guard_position.col);

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
        _ => {
            print_map(&map, &guard_position);
            panic!();
        },
    }

    if guard_is_in_puzzle_map(&map, &new_guard_position) {
        // check for an obstacle
        if map[new_guard_position.row][new_guard_position.col] == '#' || map[new_guard_position.row][new_guard_position.col] == 'O' {
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
        guard_position.row = usize::MAX;
        guard_position.col = usize::MAX;
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

