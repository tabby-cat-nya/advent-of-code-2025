// goal, figure out how many times the safe lands on 0 when following the instructions (input)
// each line has a direction L (subtracts) or R (adds)
// wraps between 99 -> 0, 99+1 = 0, 0-1=99

// my soloution plan:
// read in each line of the file, keep a count, add for R, subtract for L
// after each line, if count%100=0 then we have hit 0, count that in a seperate counter
// can also determine the resultant position by doing a count%100 at the end, maybe thats the 2nd star?

// need to figure out:
// reading in the file
// getting the first letter of the string, getting the value
// basic variables

// okay part 1 done, moving onto part 2
// after each move, lets do a modulo 100, count the hits then add/subtract the hits * 100 to get us back within the 0-99 range
//making a part 2 folder now

use std::fs;
use regex::Regex;

fn main() {
    println!("lets begin advent of code, challenge 1");

    let mut safe_value = 50;
    let mut zero_hits = 0;
    let file_path = "src/input.txt";


    for line in fs::read_to_string(file_path).unwrap().lines() {
        // println!("{line}");
        // get the letter and the number, put them in different variables
        let re = Regex::new(r"(?<direction>[RL])(?<value>\d+)").unwrap();
        let Some(caps) = re.captures(line) else{
            println!("sad cat");
            return;
        };
        // println!("The direction is: {}", &caps["direction"]);
        // println!("The value is: {}", &caps["value"]);
        if &caps["direction"] == "L"{
            safe_value -=  &caps["value"].parse::<i32>().unwrap();
        }
        else{
            safe_value +=  &caps["value"].parse::<i32>().unwrap();
        }

        if safe_value%100 == 0{
            zero_hits += 1;
        }
    }

    let true_safe_value = safe_value%100;
    println!("Final safe value: {true_safe_value}");
    println!("Total zero hits: {zero_hits}")

    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");    
    // println!("With text:\n{contents}");
}

