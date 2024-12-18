use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

use regex::Regex;

struct Button {
    x: i64,
    y: i64,
    token_cost: i64,
    presses: i64,
}

impl Button {
    fn total_cost(&self) -> i64 {
        self.presses * self.token_cost
    }
}

struct Prize {
    x: i64,
    y: i64,
}

struct ClawMachine {
    a: Button,
    b: Button,
    prize: Prize,
    token_cost: i64,
    winnable: bool,
}

impl ClawMachine {
        /*

        Systems of equations by subsitution to solve this using an example to work my brain around it:

        Given:
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Where n1 is Button A presses and n2 is Button B presses
        
        94 * n1 + 22 * n2 = 8400

        n1 = (8400 - 22 * n2) / 94


        34 * n1 + 67 * n2 = 5400

        n2 = (5400 - 34 * n1) / 67


        94 * n1 + 22 * (5400 - 34 * n1) / 67 = 8400

        67 * 94 * n1 + 22 * 5400 - 22 * 34 * n1 = 67 * 8400

        67 * 94 * n1 - 22 * 34 * n1 = 67 * 8400 - 22 * 5400

        n1 = (67 * 8400 - 22 * 5400) / (67 * 94 - 22 * 34)

        Then I substituted the values for variables below:

         */
    fn is_winnable(&mut self) -> bool {
        self.a.presses = (self.b.y * self.prize.x - self.b.x * self.prize.y) / (self.b.y * self.a.x - self.b.x * self.a.y);

        let r1 = (self.b.y * self.prize.x - self.b.x * self.prize.y) % (self.b.y * self.a.x - self.b.x * self.a.y);

        self.b.presses = (self.prize.y - self.a.y * self.a.presses) / self.b.y;

        let r2 = (self.prize.y - self.a.y * self.a.presses) % self.b.y;

        if r1 != 0 || r2 != 0 {
            return false;
        }

        self.token_cost = self.a.total_cost() + self.b.total_cost();

        self.winnable = true;

        true
    }

    fn print(&mut self, count: i64) {
        if self.is_winnable() {
            println!("{} tokens: a={} b={} total={}", count, self.a.total_cost(), self.b.total_cost(), self.token_cost);
        }
    }
}

fn parse_i64(s: &str) -> i64 {
    // skip past first two characters since input will be in the form of "X+" or "X="
    match s[2..].parse::<i64>() {
        Ok(n) => {
            return n;
        },
        Err(_e) => {
            println!("{}", s);
            panic!();
        },
    }
}

fn parse_claw_machine(claw_machine_lines: &Vec<String>) -> ClawMachine {
    if claw_machine_lines.len() != 3 {
        println!("{:?}",claw_machine_lines);
        panic!();
    }
    let button_a_line = &claw_machine_lines[0];
    let button_b_line = &claw_machine_lines[1];
    let prize_line = &claw_machine_lines[2];


    let x_regex = Regex::new(r"X\+[0-9]+").unwrap();
    let y_regex = Regex::new(r"Y\+[0-9]+").unwrap();

    const button_a_cost:i64 = 3;
    const button_b_cost:i64 = 1;

    let button_a = Button { x: parse_i64(x_regex.find(button_a_line).unwrap().as_str()), y: parse_i64(y_regex.find(button_a_line).unwrap().as_str()), token_cost: button_a_cost, presses: 0 };
    let button_b = Button { x: parse_i64(x_regex.find(button_b_line).unwrap().as_str()), y: parse_i64(y_regex.find(button_b_line).unwrap().as_str()), token_cost: button_b_cost, presses: 0 };

    let prize_x_regex = Regex::new(r"X=[0-9]+").unwrap();
    let prize_y_regex = Regex::new(r"Y=[0-9]+").unwrap();

    const adjusted_prize_position:i64 = 10000000000000; // for part 2

    let prize = Prize { x: parse_i64(prize_x_regex.find(prize_line).unwrap().as_str()) + adjusted_prize_position, y: parse_i64(prize_y_regex.find(prize_line).unwrap().as_str()) + adjusted_prize_position };

    ClawMachine {a: button_a, b: button_b, prize: prize, token_cost: 0, winnable: false}


}

fn handle_next_claw_machine(claw_machine_lines: &mut Vec<String>, claw_machine_count: &mut i64, total_cost: &mut i64) {
    let mut next_claw_machine = parse_claw_machine(claw_machine_lines);
    *claw_machine_lines = vec![];
    *claw_machine_count += 1;

    if next_claw_machine.is_winnable() {
        *total_cost += next_claw_machine.token_cost;
    }

    next_claw_machine.print(*claw_machine_count);
}

fn main() {
    let mut file_name: &str = "example";

    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() > 1 {
        file_name = &cmd_args[1];
    }

    if let Ok(lines) = read_lines(file_name.to_string()) {
        // Consumes the iterator, returns an (Optional) String
        let mut total_cost: i64 = 0;
        let mut claw_machine_lines: Vec<String> = vec![];
        let mut claw_machine_count: i64 = 0;
        for line in lines.flatten() {
            if line.len() == 0 {
                handle_next_claw_machine(&mut claw_machine_lines, &mut claw_machine_count, &mut total_cost);
            } else {
                claw_machine_lines.push(line);
            }
        }
        // handle the last claw machine
        handle_next_claw_machine(&mut claw_machine_lines, &mut claw_machine_count, &mut total_cost);

        println!("total cost: {}", total_cost);
    }

}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
