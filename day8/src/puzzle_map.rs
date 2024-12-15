use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Coordinates {
    row: usize,
    col: usize,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{},{}", self.row, self.col)
    }
}

impl std::ops::Sub<&Coordinates> for &Coordinates {
    type Output = Slope;

    fn sub (self, coord: &Coordinates) -> Self::Output {
        Slope {x: self.col as i32 - coord.col as i32, y: self.row as i32 - coord.row as i32 }
    }

    
}

impl std::ops::Sub<&Slope> for &Coordinates {
    type Output = Coordinates;

    fn sub (self, slope: &Slope) -> Self::Output {
        if self.row as i32 - slope.y < 0 {
            return Coordinates { row: usize::MAX, col: 0 }
        }
        if self.col as i32 - slope.x < 0 {
            return Coordinates { row: 0, col: usize::MAX }
        }
    
        Coordinates { row: (self.row as i32 - slope.y) as usize, col: (self.col as i32 - slope.x) as usize }
    }
}

impl std::ops::Add<&Slope> for &Coordinates {
    type Output = Coordinates;

    fn add (self, slope: &Slope) -> Self::Output {
        if self.row as i32 + slope.y < 0 {
            return Coordinates { row: usize::MAX, col: 0 }
        }
        if self.col as i32 + slope.x < 0 {
            return Coordinates { row: 0, col: usize::MAX }
        }
    
        Coordinates { row: (self.row as i32 + slope.y) as usize, col: (self.col as i32 + slope.x) as usize }
    }
}


#[derive(Clone)]
struct Slope {
    y: i32,
    x: i32,
}

impl std::ops::Mul<i32> for &Slope {
    type Output = Slope;

    fn mul (self, multiplier: i32) -> Self::Output {
        Slope { x: self.x * multiplier, y: self.y * multiplier }
    }
}

impl fmt::Display for Slope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{},{}", self.x, self.y)
    }
}

type PuzzleMapData = Vec<Vec<char>>;
type PuzzleHashMap = HashMap<char, Vec<Coordinates>>;
type PuzzleSet     = HashSet<Coordinates>;

pub struct PuzzleMap {
    area_map: PuzzleMapData,
    antenna_map: PuzzleHashMap,
    antinode_set: PuzzleSet,
    
}

fn insert_map_coords(hash_map: &mut PuzzleHashMap, key: &char, value: &Coordinates) {
    if hash_map.contains_key(key) {
        hash_map.get_mut(key).unwrap().push(Coordinates{row:value.row, col:value.col});
    } else {
        hash_map.insert(*key, vec![]);
        hash_map.get_mut(key).unwrap().push(Coordinates{row:value.row, col:value.col});
    }
}

impl PuzzleMap {

    #[inline]
    #[must_use]
    pub fn new() -> PuzzleMap {
        PuzzleMap { area_map: vec![], antenna_map: PuzzleHashMap::new(), antinode_set: PuzzleSet::new() }
    }

    pub fn get_from_file(&mut self, file_name: &str) {

        println!("get area map from file {}", file_name);
    
        if let Ok(lines) = read_lines(file_name) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.flatten() {
                self.area_map.push(line.chars().collect());
            }
        }
    }

    pub fn find_antennas(&mut self) {
        for row in 0..self.area_map.len() {
            for col in 0..self.area_map[row].len() {
                if self.area_map[row][col] != '.' {
                    // antenna here
                    insert_map_coords(&mut self.antenna_map, &self.area_map[row][col], &Coordinates{row:row, col:col});
                }
            }
        }
    }

    pub fn find_resonance_antinodes(&mut self) {

        for (_, coords) in &self.antenna_map {
            //println!("{}:", freq);
            for coord1 in coords {
                for coord2 in coords {
                    if coord1 == coord2 {
                        continue;
                    }
                    let slope: Slope = coord1 - coord2;

                    let mut direction: u8 = 0;
                    let mut multiplier = 0;

                    while direction < 2 {
                        let test_slope = &slope * multiplier;
                        match direction {
                            0 => {
                                let candidate_antinode = coord1 + &test_slope;
                                if self.coordinates_are_in_bounds(&candidate_antinode) {
                                    self.antinode_set.insert(candidate_antinode);
                                    multiplier += 1;
                                } else {
                                    multiplier = 0;
                                    direction = 1;
                                }
                            },
                            1 => {
                                let candidate_antinode = coord2 - &test_slope;
                                if self.coordinates_are_in_bounds(&candidate_antinode) {
                                    self.antinode_set.insert(candidate_antinode);
                                    multiplier += 1;
                                } else {
                                    direction = 2;
                                }
                            },
                            _ => panic!(),
                        }
                    }

                    //println!("{}", slope);
                }
            }
        }
        
    }

    pub fn find_antinodes(&mut self) {
        for (_, coords) in &self.antenna_map {
            //println!("{}:", freq);
            for coord1 in coords {
                for coord2 in coords {
                    if coord1 == coord2 {
                        continue;
                    }
                    let slope: Slope = coord1 - coord2;

                    let antinode1 = coord1 + &slope;
                    let antinode2 = coord2 - &slope;

                    // if these antinodes are valid add them to our list (if they are not already)


                    if self.coordinates_are_in_bounds(&antinode1) {
                        self.antinode_set.insert(antinode1);
                    }
                    
                    if  self.coordinates_are_in_bounds(&antinode2) {
                        self.antinode_set.insert(antinode2);
                    }

                    //println!("{}", slope);
                }
            }
        }
    }

    pub fn mark_antinodes(&mut self) {
        for coord in &self.antinode_set {
            if self.area_map[coord.row][coord.col] == '.' {
                self.area_map[coord.row][coord.col] = '#';
            }
            else {
                println!("failed to mark {}", coord);
            }
        }
    }

    fn coordinates_are_in_bounds(&self, coords: &Coordinates) -> bool {
        return coords.row < self.area_map.len() && coords.col < self.area_map[0].len();
    }

    pub fn print(&self) {
        // reset console
        print!("\x1B[2J\x1B[1;1H");

        for row in 0..self.area_map.len() {
            for col in 0..self.area_map[0].len() {
                print!("{} ", self.area_map[row][col]);
            }
            println!("");
        }

        println!("antennas:");
        
        for (freq, coords) in &self.antenna_map {
            print!("{}: ", freq);
            for coord in coords {
                print!("{} ", coord);
            }
            println!("\n");
        }


        println!("antinodes:");

        for coord in &self.antinode_set {
                print!("{} ", coord);
        }
        println!("\n");

        println!("antinode_count {}", self.antinode_set.len());
    }

}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
