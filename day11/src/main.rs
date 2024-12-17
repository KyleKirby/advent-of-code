use std::fs;
use std::env;
use std::env::Args;
use std::collections::HashMap;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

type Stone = u64;
type StoneCount = u64;
type Stones = HashMap<Stone, StoneCount>;

fn add_stone(stones: &mut Stones, stone: &Stone, count: StoneCount) {
    if stones.contains_key(&stone) {
        *stones.get_mut(&stone).unwrap() += count;
    } else {
        stones.insert(*stone, count);
    }
}

#[allow(dead_code)]
fn print_stones(stones: &Stones) {
    for (stone, count) in stones {
        print!("{} {} ", stone, count);
    }
    println!("\n");
}

fn init_stones(message: String) -> Stones {
    let message_stones: Vec<&str> = message.split(" ").collect();

    let mut stones: Stones = Stones::new();

    for stone in message_stones {
        if let Ok(n) = stone.parse::<Stone>() {
            add_stone(&mut stones, &n, 1u64);
        }
    }

    stones
}


fn blink(stones: &mut Stones) {
    let mut new_stones = Stones::new();

    for (stone, count) in stones.iter_mut() {
        if *count == 0 {
            // skip entries with no stones
            continue;
        }

        if *stone == 0 {
            // replace 0 with 1
            add_stone(&mut new_stones, &1, *count);
            *count = 0;
        } else {
            let stone_string = stone.to_string();
            let stone_string_len = stone_string.len();
            if stone_string_len%2 == 0 {
                // even number of digits
    
                let half = stone_string_len / 2;
                let chars: Vec<char> = stone_string.chars().collect();
                let mut s1 = "".to_string();
                let mut s2 = "".to_string();
    
                // create two strings and put half of the characters in each string in order to split the stone's digits in half
    
                for i in 0..half {
                    s1.push(chars[i]);
                }

                for i in half..stone_string_len {
                    s2.push(chars[i]);
                }
                
                if let Ok(n) = s1.parse::<u64>() {
                    add_stone(&mut new_stones, &n, *count);
                } else {
                    // failed to parse?..
                    panic!();
                }

                if let Ok(n) = s2.parse::<u64>() {
                    add_stone(&mut new_stones, &n, *count);
                } else {
                    // failed to parse?..
                    panic!();
                }

                *count = 0;
            } else {
                // replace with the stone's number multiplied by 2024
                add_stone(&mut new_stones, &(stone*2024), *count);
                *count = 0;
            }
        }
    }

    for (stone, count) in new_stones {
        add_stone(stones, &stone, count);
    }

}

fn blink_until(stones: Stones, blinks_remaining: u8) -> u64 {
    let mut handles = vec![];
    let mut thread_count = 0;

    for (stone, count) in stones {
        let mut thread_stones = Stones::new();
        add_stone(&mut thread_stones, &stone, count);
        let handle = thread::spawn(move || -> u64 {
            for _i in 0..blinks_remaining {
                blink(&mut thread_stones);
                println!("thread {} blinks: {}", thread_count, _i);
            }

            let mut total: u64 = 0;

            for (_, count) in thread_stones {
                total += count;
            }

            total
        });
        handles.push(handle);
        thread_count += 1;
    }

    let mut result: u64 = 0;

    for handle in handles {
        result += handle.join().unwrap();
    }

    result
}

fn main() {
    let message: String = get_string_from_file(env::args());

    let stones: Stones = init_stones(message);

    const MAX_BLINKS: u8 = 75;

    println!("Initial arrangement:");
    print_stones(&stones);

    let start = SystemTime::now();

    let total_stones = blink_until(stones, MAX_BLINKS);

    let end = SystemTime::now();

    println!("stone count: {}", total_stones);

    let start_since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let end_since_epoch = end
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    println!("total time in ms: {}", end_since_epoch.as_millis() - start_since_epoch.as_millis());

}


fn get_string_from_file(args: Args) -> String {
    let mut file_name: &str = "example";

    let cmd_args: Vec<String> = args.collect();

    if cmd_args.len() > 1 {
        file_name = &cmd_args[1];
    }

    fs::read_to_string(file_name).expect("Error reading file")
}
