
use std::env;
#[path = "../../utils/src/trails.rs"] mod trails;
use trails::TrailFinder;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut trail_finder: TrailFinder = TrailFinder::new();

    trail_finder.get_from_file(env::args());

    trail_finder.find_all_trail_head_peaks();

    trail_finder.print();

    
}







