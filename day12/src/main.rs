


use std::env;

mod farm;
use farm::Farm;

fn main() {
    
    env::set_var("RUST_BACKTRACE", "1");

    let mut farm: Farm = Farm::new();

    farm.get_from_file(env::args());

    farm.find_gardens();

    farm.calculate_perimeters();

    farm.print();

}
