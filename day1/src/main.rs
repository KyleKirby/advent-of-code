use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    let mut counts: HashMap<i32, i32> = HashMap::new();

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
            let numbers: Vec< _> = line.split("   ").collect();

            match numbers[0].to_string().parse::<i32>() {
                Ok(n) => {
                    left.push(n);
                },
                Err(e) => println!("{}", e),
            }

            match numbers[1].to_string().parse::<i32>() {
                Ok(n) => {
                    right.push(n);

                    if counts.contains_key(&n) {
                        *counts.get_mut(&n).unwrap() += 1;
                    } else {
                        counts.insert(n, 1);
                    }
                },
                Err(e) => println!("{}", e),
            }
        }
    }

    let mut sum = 0;

    println!("left  {:?}", left);
    println!("right {:?}", right);

    left.sort();
    right.sort();

    println!("left sorted  {:?}", left);
    println!("right sorted {:?}", right);

    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
        println!("{} - {} = {}", left[i], right[i], (left[i] - right[i]).abs());
    }

    println!("sum {}", sum);

    let mut similarity = 0;

    for i in left {
        if counts.contains_key(&i) {
            similarity += i * *counts.get_mut(&i).unwrap();
        }
    }

    println!("counts {:?}", counts);

    println!("similarity {}", similarity);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}