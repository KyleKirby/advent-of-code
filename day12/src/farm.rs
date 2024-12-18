
use std::env::Args;
#[path = "../../utils/src/puzzle_map.rs"] mod puzzle_map;
use puzzle_map::PuzzleMap;
use puzzle_map::Coordinates;
use std::collections::HashSet;
use std::collections::HashMap;

type Region = HashSet<Coordinates>;

#[derive(Eq, Hash, PartialEq, Clone)]
enum Sides{
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
struct Garden {
    plant_type: char,
    region: Region,
    perimeter: usize,
    plant_list: Vec<Coordinates>,
    sides: HashMap<Sides, HashSet<Coordinates>>,
}

impl Garden {
    #[inline]
    #[must_use]
    pub fn new(plant: char) -> Garden {
        Garden { region: Region::new(), plant_type: plant, perimeter: 0, plant_list: vec![], sides: HashMap::new() }
    }

    #[allow(dead_code)]
    fn plant_is_in_same_region(&self, plant_coord_option: Option<Coordinates>) -> bool {
        if let Some(plant_coord) = plant_coord_option {
            return self.region.contains(&plant_coord);
        }
        false
    }
    
    // find other plants adjacent to the specified plant that also have an outer edge with the specified side
    fn get_plants_sharing_side(&self, plant: &Coordinates, side: &Sides) -> Vec<Coordinates> {
        let mut plants = vec![*plant];
        match side {
            Sides::Up | Sides::Down => {
                // traverse left
                let mut plant_iter = plant.clone();
                while let Some(other_plant) = plant_iter.left(1) {
                    if self.sides.get(side).unwrap().contains(&other_plant)  {
                        plants.push(other_plant);
                        plant_iter = other_plant
                    } else {
                        break;
                    }
                }

                // traverse right
                let mut plant_iter = plant.clone();
                while let Some(other_plant) = plant_iter.right(1) {
                    if self.sides.get(side).unwrap().contains(&other_plant) {
                        plants.push(other_plant);
                        plant_iter = other_plant
                    } else {
                        break;
                    }
                }
            },
            Sides::Left | Sides::Right => {
                // traverse up
                let mut plant_iter = plant.clone();
                while let Some(other_plant) = plant_iter.up(1) {
                    if self.sides.get(side).unwrap().contains(&other_plant) {
                        plants.push(other_plant);
                        plant_iter = other_plant
                    } else {
                        break;
                    }
                }

                // traverse down
                let mut plant_iter = plant.clone();
                while let Some(other_plant) = plant_iter.down(1) {
                    if self.sides.get(side).unwrap().contains(&other_plant) {
                        plants.push(other_plant);
                        plant_iter = other_plant
                    } else {
                        break;
                    }
                }
            },
        }

        plants
    }


    #[allow(dead_code)]
    fn calculate_sides(&self) -> usize {
        let mut count: usize = 0;

        for (side, side_set) in &self.sides {
            let mut compared_set: HashSet<Coordinates> = HashSet::new();

            for plant in side_set {
                if compared_set.contains(plant) {
                    continue;
                }

                let plants_sharing_side = self.get_plants_sharing_side(plant, side);

                for _plant in plants_sharing_side {
                    compared_set.insert(_plant);
                }

                count += 1;
            }
        }

        count
    }

    #[allow(dead_code)]
    fn calculate_perimeter(&mut self) {
        self.sides.insert(Sides::Up, HashSet::new());
        self.sides.insert(Sides::Down, HashSet::new());
        self.sides.insert(Sides::Left, HashSet::new());
        self.sides.insert(Sides::Right, HashSet::new());

        for plant_coord in &self.region {
            let mut perimeter = 0;
            if !self.plant_is_in_same_region(plant_coord.up(1)) {
                self.sides.get_mut(&Sides::Up).unwrap().insert(*plant_coord);
                perimeter += 1;
            }
            if !self.plant_is_in_same_region(plant_coord.down(1)) {
                self.sides.get_mut(&Sides::Down).unwrap().insert(*plant_coord);
                perimeter += 1;

            }
            if !self.plant_is_in_same_region(plant_coord.left(1)) {
                self.sides.get_mut(&Sides::Left).unwrap().insert(*plant_coord);
                perimeter += 1;

            }
            if !self.plant_is_in_same_region(plant_coord.right(1)) {
                self.sides.get_mut(&Sides::Right).unwrap().insert(*plant_coord);
                perimeter += 1;
            }
            self.perimeter += perimeter;
        }
    }


    fn add(&mut self, plant: Coordinates) -> bool {
        if self.region.insert(plant) {
            self.plant_list.push(plant);

            return true;
        }
        false
    }

}

type GardenId = usize;

pub struct Farm {
    area_map: PuzzleMap,
    gardens: Vec<Garden>,
    plants_gardens_map: HashMap<Coordinates, GardenId>,
}

impl Farm {

    #[inline]
    #[must_use]
    pub fn new() -> Farm {
        Farm { area_map: PuzzleMap::new(), gardens: vec![], plants_gardens_map: HashMap::new() }
    }

    #[allow(dead_code)]
    pub fn get_from_file(&mut self, args: Args) {
        self.area_map.get_from_file(args);
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.area_map.print();

        println!("gardens: {}", self.gardens.len());

        let mut total_perimeter_cost = 0;
        let mut total_side_cost = 0;
        for garden in &self.gardens {
            let garden_sides = garden.calculate_sides();
            let garden_perimeter_cost = garden.perimeter * garden.region.len(); // perimeter * area
            let garden_side_cost = garden_sides * garden.region.len(); // # sides * area
            println!("{} perimeter cost: {} * {} = {}", garden.plant_type, garden.region.len(), garden.perimeter, garden_perimeter_cost);
            println!("{} sides cost    : {} * {} = {}", garden.plant_type, garden.region.len(), garden_sides, garden_side_cost);
            total_perimeter_cost += garden_perimeter_cost;
            total_side_cost += garden_side_cost;
        }

        println!("gardens:    {}", self.gardens.len());
        println!("total perimeter cost: {}", total_perimeter_cost);
        println!("total side cost     : {}", total_side_cost);
    }

    fn plant_type_matches(&self, plant: char, other_plant_coord: &Coordinates) -> bool {
        self.area_map.coordinates_are_in_bounds(other_plant_coord) && self.area_map[other_plant_coord] == plant
    }

    fn add_connected_plant(&mut self, plant: char, plant_coord_option: Option<Coordinates>, garden_id: GardenId) {
        if let Some(plant_coord) = plant_coord_option {
            if self.plant_type_matches(plant, &plant_coord) && self.gardens[garden_id].add(plant_coord) {
                self.plants_gardens_map.insert(plant_coord, garden_id);
                self.add_connected_plants(plant, &plant_coord, garden_id);
            }
        }
    }

    fn add_connected_plants(&mut self, plant: char, plant_coord: &Coordinates, garden_id: GardenId) {
        self.add_connected_plant(plant, plant_coord.up(1), garden_id);
        self.add_connected_plant(plant, plant_coord.down(1), garden_id);
        self.add_connected_plant(plant, plant_coord.left(1), garden_id);
        self.add_connected_plant(plant, plant_coord.right(1), garden_id);
    }

    
    #[allow(dead_code)]
    pub fn calculate_perimeters(&mut self) {
        for garden in &mut self.gardens {
            garden.calculate_perimeter();
        }
    }

    pub fn find_gardens(&mut self) {
        let map_size = self.area_map.len();
        let map_width = self.area_map.width();
        
        for row in 0..map_size {
            for col in  0..map_width {
                let plant = self.area_map[row][col];
                let plant_coord = Coordinates::new(row, col);

                if self.plants_gardens_map.contains_key(&plant_coord) {
                    // already processed this plant
                    continue;
                }

                let mut garden_id_option: Option<&GardenId> = None;

                let mut garden_id: GardenId = self.gardens.len();

                // try to find an adjacent garden
                if let Some(plant_coord) = plant_coord.up(1) {
                    if self.plant_type_matches(plant, &plant_coord) {
                        garden_id_option = self.plants_gardens_map.get(&plant_coord);
                    }
                } else if let Some(plant_coord) = plant_coord.down(1) {
                    if self.plant_type_matches(plant, &plant_coord) {
                        garden_id_option = self.plants_gardens_map.get(&plant_coord);
                    }
                } else if let Some(plant_coord) = plant_coord.right(1) {
                    if self.plant_type_matches(plant, &plant_coord) {
                        garden_id_option = self.plants_gardens_map.get(&plant_coord);
                    }
                } else if let Some(plant_coord) = plant_coord.left(1) {
                    if self.plant_type_matches(plant, &plant_coord) {
                        garden_id_option = self.plants_gardens_map.get(&plant_coord);
                    }
                } 

                if garden_id_option == None {
                    // did not find an adjacent garden
                    let mut garden: Garden = Garden::new(plant);
                    garden.region.insert(plant_coord);

                    self.gardens.push(garden);
                } else {
                    garden_id = *garden_id_option.unwrap();
                }

                self.plants_gardens_map.insert(plant_coord, garden_id);
                self.add_connected_plants(plant, &plant_coord, garden_id);
            }
        }
    }


}
