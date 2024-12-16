
use std::env;
mod disk;
use disk::DiskManager;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let cmd_args: Vec<String> = env::args().collect();

    let mut source_file = "example".to_string();

    if cmd_args.len() > 1 {
        source_file = cmd_args[1].clone();
    }

    let mut disk_manager: DiskManager = DiskManager::new();

    disk_manager.get_from_file(&source_file);

    disk_manager.parse_disk_string();


    disk_manager.print();

    //disk_manager.compact_blocks(); // part 1

    disk_manager.compact_files(); // part 2

    disk_manager.print();


    
}







