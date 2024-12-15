use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod permutations;
use permutations::ToPermutationsWithReplacement;

fn main() {
    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String

        let mut sum = 0;
        for line in lines.flatten() {
            let (test_value, inputs) = parse_line(line);

            if has_valid_equation(&test_value, &inputs) {
                sum += test_value;
            }

            //println!("{} : {:?}", test_value, inputs);
        }

        println!("sum {}", sum);

    }
}

fn has_valid_equation(test_value: &i64, inputs: &Vec<i64>) -> bool {
    assert!(inputs.len() >= 2, "inputs length < 2");

    let possible_operators: Vec<char> = vec!['+', '*', '|'];
    let num_operators = inputs.len() - 1;

    let operator_permutations = (0..possible_operators.len()).permutations_with_replacement(num_operators);

    for operator_permutation in operator_permutations {
        let mut total: i64 = 0;

        //print!("test: {} ; {} ", test_value, inputs[0]);

        for i in 0..operator_permutation.len() {

            //print!("{} {} ", possible_operators[operator_permutation[i]], inputs[i+1]);

            match possible_operators[operator_permutation[i]] {
                '+' => {

                    if i == 0 {
                        total = inputs[0] + inputs[1];
                    } else {
                        total += inputs[i+1];
                    }
                },
                '*' => {
                    if i == 0 {
                        total = inputs[0] * inputs[1];
                    } else {
                        total *= inputs[i+1];
                    }
                },
                '|' => {
                    if i == 0 {
                        total = concat_numbers(inputs[0], inputs[1]);
                    } else {
                        total = concat_numbers(total, inputs[i+1]);
                    }
                },
                _ => panic!(),
            }
        }


        if total == *test_value {
            //println!("= {} , true! \n", total);

            return true;
        }
        else {
            //println!("= {} , false! \n", total);

        }
    }

    return false;
}

fn concat_numbers(num1: i64, num2: i64) -> i64 {
    let mut concat_result: String = num1.to_string();
    concat_result.push_str(&num2.to_string());

    match concat_result.parse::<i64>() {
        Ok(concat_number) => {
            return concat_number;
        },
        Err(_) => {
            panic!();
        },                 
    }
}

fn parse_line(line: String) -> (i64, Vec<i64>) {
    //println!("{}", line);
    let numbers: Vec<&str> = line.split(":").collect();
    match numbers[0].to_string().parse::<i64>() {
        Ok(test_value) => {
            let inputs_str: Vec<&str> = numbers[1].split(" ").collect();
            let mut inputs: Vec<i64> = vec![];
            for input_str in inputs_str {
                if input_str.len() == 0 {
                    // skip empty strings
                    continue;
                }
                match input_str.to_string().parse::<i64>() {
                    Ok(input) => {
                        inputs.push(input);
                    },
                    Err(e) => println!("{}", e),
                }
            }
            return (test_value, inputs);
        },
        Err(e) => println!("{}", e),
    }
    panic!();
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
