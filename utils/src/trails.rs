
mod puzzle_map;
use puzzle_map::PuzzleMap;
use puzzle_map::Coordinates;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env::Args;

type PeakMap = HashMap<Coordinates, HashSet<Coordinates>>;
type RatingsMap = HashMap<Coordinates, usize>;

pub struct TrailFinder {
    area_map: PuzzleMap,
    trail_head_peaks: PeakMap,
    trail_head_ratings: RatingsMap,
}

impl TrailFinder {
    #[inline]
    #[must_use]
    pub fn new() -> TrailFinder {
        TrailFinder { area_map: PuzzleMap::new(), trail_head_peaks: PeakMap::new(), trail_head_ratings: RatingsMap::new() }
    }

    #[allow(dead_code)]
    pub fn get_from_file(&mut self, args: Args) {
        self.area_map.get_from_file(args);
    }

    fn get_total_score(&self) -> usize {
        let mut total_score: usize = 0;
        for (_, peaks) in &self.trail_head_peaks {
            total_score += peaks.len();
        }

        total_score
    }

    fn get_total_rating(&self) -> usize {
        let mut total_ratings: usize = 0;

        for (_, rating) in &self.trail_head_ratings {
            total_ratings += rating;
        }

        total_ratings
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.area_map.print();

        println!("total score: {}", self.get_total_score());
        println!("total rating: {}", self.get_total_rating());

    }

    fn step(&mut self, trail_head: &Coordinates, last_coord: &Coordinates, next_coord_option: &Option<Coordinates>) {
        if *next_coord_option == None {
            return;
        }

        let next_coord: Coordinates = next_coord_option.unwrap();

        if !self.area_map.coordinates_are_in_bounds(&next_coord) {
            return;
        }

        if self.area_map[&next_coord] as u8 == self.area_map[last_coord] as u8 + 1 {
            self.find_peak(trail_head, &next_coord);
        }
    }

    fn find_peak(&mut self, trail_head: &Coordinates, coord: &Coordinates) {
        match self.area_map[coord] {
            '9' => {
                // We've reached a peak
                self.trail_head_peaks.get_mut(trail_head).unwrap().insert(*coord);
                *self.trail_head_ratings.get_mut(trail_head).unwrap() += 1;
            },
            _ => {
                // Keep going
                self.step(trail_head, coord, &coord.up(1));
                self.step(trail_head, coord, &coord.down(1));
                self.step(trail_head, coord, &coord.left(1));
                self.step(trail_head, coord, &coord.right(1));
            }
        }
    }



    fn find_trail_head_peaks(&mut self, trail_head: &Coordinates) {
        self.trail_head_peaks.insert(*trail_head, HashSet::new());
        self.trail_head_ratings.insert(*trail_head, 0);

        self.find_peak(trail_head, trail_head);
    }

    #[allow(dead_code)]
    pub fn find_all_trail_head_peaks(&mut self) {
        for row in 0..self.area_map.len() {
            for col in 0..self.area_map[row].len() {
                if self.area_map[row][col] == '0' {
                    self.find_trail_head_peaks(&Coordinates::new(row, col));
                }
            }
        }
    }

}
