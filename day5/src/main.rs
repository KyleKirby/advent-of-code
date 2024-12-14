use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::Ordering;

type RuleHashMap = HashMap<i32, HashMap<i32, Ordering>>;

fn main() {

    let mut rules: RuleHashMap = HashMap::new();
    let mut section = 0;
    let mut sum_middle_numbers_of_correct_updates = 0;
    let mut sum_middle_numbers_of_incorrect_updates = 0;

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            match section {
                0 => {
                    if line.len() == 0 {
                        // blank lines means this is the end of the first section
                        section = 1;
                        print_rules(&rules);

                        continue;
                    }

                    // new rule
                    add_rule(&mut rules, line);
                },
                1 => {
                    // new update
                    let update = get_update_vec(line);

                    let mut sorted_update = update.clone();

                    sorted_update.sort_by(|a, b| {
                        if let Some(rule) = rules.get(a) {
                            if let Some(sub_rule) = rule.get(b) {
                                return *sub_rule;
                            }
                        }
                        // there is no rule for this pairing
                        return Ordering::Equal;
                    });

                    println!("update        {:?}", update);
                    println!("sorted_update {:?}", sorted_update);

                    if ordered_vecs_are_equal(&update, &sorted_update) {
                        // this update was already in the correct order
                        sum_middle_numbers_of_correct_updates += update[update.len() / 2];
                    } else {
                        sum_middle_numbers_of_incorrect_updates += sorted_update[sorted_update.len() / 2];
                    }

                },
                _ => panic!(),
            }
        }
    }

    println!("sum_middle_numbers_of_correct_updates {}", sum_middle_numbers_of_correct_updates);
    println!("sum_middle_numbers_of_incorrect_updates {}", sum_middle_numbers_of_incorrect_updates);
}

fn get_update_vec(update_line: String) -> Vec<i32> {
    let update_string: Vec< _> = update_line.split(",").collect();

    let mut update: Vec<i32> = vec![];

    for num_str in update_string {
        match num_str.to_string().parse::<i32>() {
            Ok(num) => {
                update.push(num);
            },
            Err(e) => println!("{}", e),
        }
    }

    return update;
}

fn ordered_vecs_are_equal(vec1: &Vec<i32>, vec2: &Vec<i32>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }

    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            return false;
        }
    }
    return true;
}

fn print_rules(rules: &RuleHashMap) {
    for (rules_key, sub_rules) in rules {
        println!("{}:\n", rules_key);
        for (sub_rules_key, ordering) in sub_rules {
            match ordering {
                Ordering::Less => {
                    println!("    {}: {}\n", sub_rules_key, "BEFORE");
                },
                Ordering::Greater => {
                    println!("    {}: {}\n", sub_rules_key, "AFTER");
                },
                Ordering::Equal => {
                    println!("    {}: {}\n", sub_rules_key, "EQUAL");
                },
            }
        }
    }
}

fn add_rule(rules: &mut RuleHashMap, rule: String) {
    let (left_number, right_number) = parse_rule(rule);

    if rules.contains_key(&left_number) {
        rules.get_mut(&left_number).unwrap().insert(right_number, Ordering::Less);
    } else {
        rules.insert(left_number, HashMap::new());
        rules.get_mut(&left_number).unwrap().insert(right_number, Ordering::Less);
    }

    if rules.contains_key(&right_number) {
        rules.get_mut(&right_number).unwrap().insert(left_number, Ordering::Greater);
    } else {
        rules.insert(right_number, HashMap::new());
        rules.get_mut(&right_number).unwrap().insert(left_number, Ordering::Greater);
    }

}

fn parse_rule(rule: String) -> (i32, i32) {
    let numbers: Vec< _> = rule.split("|").collect();

    // parse both numbers for the rule
    match numbers[0].to_string().parse::<i32>() {
        Ok(left_number) => {
            match numbers[1].to_string().parse::<i32>() {
                Ok(right_number) => {
                    return (left_number, right_number);
                },
                Err(e) => println!("{}", e),
            }
        },
        Err(e) => println!("{}", e),
    }

    return (-1, -1);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}