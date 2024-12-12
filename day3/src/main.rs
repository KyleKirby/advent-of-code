use std::fs;

use regex::Regex;

fn main() {
    let message: String = fs::read_to_string("input").expect("Error reading file");
    
    
    println!("sum part 1 {}", sum_products_in_message(&message));

    // rust can't do look around so remove everything between don't() and do()
    let re_part2 = Regex::new(r"(don't\(\))(.+?\n*)+?(do\(\))").unwrap();
    let modified_message = re_part2.replace_all(&message, "");

    println!("sum part 2 {}", sum_products_in_message(&modified_message.to_string()));
}

fn sum_products_in_message(message: &String) -> i32 {
    let mut sum = 0;

    let product_regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let int_regex = Regex::new(r"[0-9]*").unwrap();

    //let mut match_index = 0;
    for iter in product_regex.find_iter(&message)  {
        // regex results in the form of mult(n1, n2)
        let mut product = 1;
        for iter2 in int_regex.find_iter(iter.as_str())  {
            // regex results in the form of individual numbers from the last regex result
            //println!("{:?}", iter2.as_str());
            let iter2_str = iter2.as_str();
            if iter2_str.len() == 0 {
                // some of these regex results are empty strings
                continue;
            }
            match iter2_str.parse::<i32>() {
                Ok(n) => {
                    product *= n;
                },
                Err(_e) => {
                    //println!("{}", e);
                },
            }

        }
        //println!("{} {:?} = {}", match_index, iter.as_str(), product);

        //match_index += 1;
        sum += product;
    }

    //println!("number of matches: {}", match_index);

    return sum;
}
