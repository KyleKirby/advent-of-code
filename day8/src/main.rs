
use std::env;
mod puzzle_map;
use puzzle_map::PuzzleMap;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let cmd_args: Vec<String> = env::args().collect();

    let mut source_file = "example".to_string();

    if cmd_args.len() > 1 {
        source_file = cmd_args[1].clone();
    }

    let mut area_map: PuzzleMap = PuzzleMap::new();

    area_map.get_from_file(&source_file);

    area_map.find_antennas();

    //area_map.find_antinodes(); // part 1

    area_map.find_resonance_antinodes(); // part 2

    area_map.mark_antinodes();

    area_map.print();



    


    
}







