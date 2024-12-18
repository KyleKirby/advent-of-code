use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::{thread, time};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Coordinates {
        Coordinates {x: x, y: y}
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    fn new(x: i32, y: i32) -> Velocity {
        Velocity {x: x, y: y}
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Robot {
    start_pos: Coordinates,
    current_pos: Coordinates,
    velocity: Velocity,
}

impl Robot {

    fn new(new_pos: Coordinates, new_velocity: Velocity) -> Robot {
        Robot {start_pos: new_pos, current_pos: new_pos, velocity: new_velocity}
    }

    fn advance(&mut self, num_times_to_advance: i32, x_max: i32, y_max: i32) {
        self.current_pos.x = (self.current_pos.x + self.velocity.x  * num_times_to_advance) % x_max;
        self.current_pos.y = (self.current_pos.y + self.velocity.y  * num_times_to_advance) % y_max;

        if self.current_pos.x < 0 {
            self.current_pos.x += x_max;
        }
        if self.current_pos.y < 0 {
            self.current_pos.y += y_max;
        }
    }

}

struct Arena {
    height: u8,
    width: u8,
    robots: Vec<Robot>,
}

use std::fs::OpenOptions;
use std::io::prelude::*;

impl Arena {

    fn print_to_file(&self, iteration: i32, file: &mut File) {
        if let Err(e) = writeln!(file, "==========================================================================================================================") {
            eprintln!("Couldn't write to file: {}", e);
        }
        

        if let Err(e) = writeln!(file, "after {} seconds", iteration) {
            eprintln!("Couldn't write to file: {}", e);
        }


        let mut map: Vec<Vec<i32>> = vec![];
        for _ in 0..self.height {
            //let row: Vec<i32> = vec![];
            map.push(vec![0; self.width as usize]);
        }

        for robot in &self.robots {
            map[robot.current_pos.y as usize][robot.current_pos.x as usize] += 1;
        }

        for row in map {
            for col in row {
                if col == 0 {
                    if let Err(e) = write!(file, ".") {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
                else {
                    if let Err(e) = write!(file, "#") {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }
            if let Err(e) = writeln!(file, " ") {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }

    fn print(&self, iteration: i32) {
        println!("==========================================================================================================================");
        print!("\x1B[2J\x1B[1;1H");
        println!("after {} seconds", iteration);

        let mut map: Vec<Vec<i32>> = vec![];
        for _ in 0..self.height {
            //let row: Vec<i32> = vec![];
            map.push(vec![0; self.width as usize]);
        }

        for robot in &self.robots {
            map[robot.current_pos.y as usize][robot.current_pos.x as usize] += 1;
        }

        for row in map {
            for col in row {
                if col == 0 {
                    print!(".");
                }
                else {
                    print!("{}", col);
                }
            }
            println!(" ");
        }
        println!(" ");
        println!(" ");
    }

    


    fn advance_robots(&mut self, num_times_to_advance: i32) {
        /* 
        let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("tree")
        .unwrap();
        */

        for i in 0..num_times_to_advance {
            for robot in &mut self.robots {
                robot.advance(1, self.width.into(), self.height.into());
            }

            //print!("\x1B[2J\x1B[1;1H");
            //println!("after {} seconds", i);
            //self.print_to_file(i, &mut file);
            //self.print(i);
            //thread::sleep(time::Duration::from_millis(300));
        }
        
    }

    fn safety_factor(&self) -> u32 {
        let mid_height: i32 = (self.height / 2).into();
        let mid_width: i32 = (self.width / 2).into();

        println!("mid_width {} mid_height {}", mid_width, mid_height);
        
        let mut sum0: u32 = 0;
        let mut sum1: u32 = 0;
        let mut sum2: u32 = 0;
        let mut sum3: u32 = 0;

        for robot in &self.robots {
            
            // println!("start {},{} velocity {},{}", robot.start_pos.x, robot.start_pos.y, robot.velocity.x, robot.velocity.y);
            // println!("now   {},{}", robot.current_pos.x, robot.current_pos.y);

            if robot.current_pos.x == mid_width || robot.current_pos.y == mid_height {
                // robot is in the middle so don't count it
                println!("not counted");
                continue;
            }

            // if robot.start_pos.x != 2 || robot.start_pos.y != 4 {
            //     continue;
            // }


            
            if robot.current_pos.x < mid_width && robot.current_pos.y < mid_height {
                sum0 += 1;
            } else if robot.current_pos.x > mid_width && robot.current_pos.y < mid_height {
                sum1 += 1;
            } else if robot.current_pos.x < mid_width && robot.current_pos.y > mid_height {
                sum2 += 1;
            } else if robot.current_pos.x > mid_width && robot.current_pos.y > mid_height {
                sum3 += 1;
            }
        }

        println!("{} {} {} {}", sum0, sum1, sum2, sum3);

        sum0 * sum1 * sum2 * sum3
    }

    

}

fn parse_i32(s: &str) -> i32 {
    // skip past first two characters since input will be in the form of "X+" or "X="
    match s.parse::<i32>() {
        Ok(n) => {
            return n;
        },
        Err(_e) => {
            println!("{}", s);
            panic!();
        },
    }
}

fn main() {
    let mut file_name: &str = "example";

    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() > 1 {
        file_name = &cmd_args[1];
    }

    let mut num_times_to_advance: i32 = 5;


    if cmd_args.len() > 2 {
        num_times_to_advance = parse_i32(&cmd_args[2]);
    }

    // default is example, so use these dimensions
    let mut arena = Arena { width: 11, height: 7, robots: vec![] };
    
    if file_name == "input" {
        arena.width = 101;
        arena.height = 103;
    }

    if let Ok(lines) = read_lines(file_name.to_string()) {
        // Consumes the iterator, returns an (Optional) String

        for line in lines.flatten() {
            /*
            line in the form of:
            p=0,4 v=3,-3
             */
            let split: Vec<&str> = line.split(" ").collect();
            let pos_str = split[0];
            let v_str = split[1];
            let positions: Vec<&str> = pos_str[2..].split(",").collect();
            let velocities: Vec<&str> = v_str[2..].split(",").collect();

            arena.robots.push(Robot::new(Coordinates::new(parse_i32(positions[0]), parse_i32(positions[1])),
                                Velocity::new(parse_i32(velocities[0]), parse_i32(velocities[1]))
                                ))

        }


        arena.advance_robots(num_times_to_advance);
        
        println!("safety factor after {} seconds: {}", num_times_to_advance, arena.safety_factor());
    } else {
        panic!();
    }

}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}