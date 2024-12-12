use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
enum DIRECTION {
    INCREASING,
    DECREASING,
    UNSET,
}

fn main() {
    let mut safe_reports = 0;
    let mut safe_reports_part_2 = 0;

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for report_line in lines.flatten() {
            let report: Vec<String> = report_line.split(" ").map(str::to_string).collect();
            if is_report_safe(report.clone()) {
                safe_reports += 1;
                safe_reports_part_2 += 1;
            } else {
                for i in 0..report.len() {
                    // try removing this number and see if the report would be safe
                    let mut modified_report = report.clone();
                    modified_report.remove(i);
                    if is_report_safe(modified_report) {
                        safe_reports_part_2 += 1;
                        break;
                    }
                }
            }

        }
    }

    println!("safe_reports {}", safe_reports);
    println!("safe_reports_part_2 {}", safe_reports_part_2);
}

fn is_report_safe(report: Vec<String>) -> bool {
    let mut num_levels = 0;
    let mut last_number = 0;
    let mut report_direction = DIRECTION::UNSET;

    for number in &report {
        num_levels += 1;
        if num_levels == 1 {
            match number.to_string().parse::<i32>() {
                Ok(n) => {
                    last_number = n;
                },
                Err(e) => {
                    println!("{}", e);
                    return false;
                },
            }
            // no comparison needed
            continue;
        }

        match number.to_string().parse::<i32>() {
            Ok(n) => {
                if n == last_number || (n - last_number).abs() > 3 {
                    // unsafe
                    return false;
                }

                if report_direction == DIRECTION::UNSET {
                    // set direction
                    if n > last_number {
                        report_direction = DIRECTION::INCREASING;
                    } else {
                        report_direction = DIRECTION::DECREASING;
                    }
                } else {
                    // check direciton
                    if n > last_number && report_direction != DIRECTION::INCREASING {
                        // unsafe
                        return false;
                    } else if n < last_number && report_direction != DIRECTION::DECREASING {
                        // unsafe
                        return false;
                    }
                }
                    
                last_number = n;
            },
            Err(e) => {
                println!("{}", e);
                return false;
            },
        }
    }

    // no unsafe conditions found
    println!("{:?} safe", report);
    return true;
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}