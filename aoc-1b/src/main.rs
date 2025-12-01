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
    println!("lets begin advent of code, challenge 1, part 2");

    let mut safe_value: i32 = 50;
    let mut zero_hits = 0;
    let file_path = "src/input.txt";
    // let file_path = "src/demo.txt";


    for line in fs::read_to_string(file_path).unwrap().lines() {
        println!("{line}");
        print!("start {safe_value}, ");
        // get the letter and the number, put them in different variables
        let re = Regex::new(r"(?<direction>[RL])(?<value>\d+)").unwrap();
        let Some(caps) = re.captures(line) else{
            println!("sad cat");
            return;
        };
        // println!("The direction is: {}", &caps["direction"]);
        // println!("The value is: {}", &caps["value"]);

        // if safe_value == 0{
        //     zero_hits -= 1;
        // }

        if &caps["direction"] == "L"{
            safe_value -=  &caps["value"].parse::<i32>().unwrap();
        }
        else{
            safe_value +=  &caps["value"].parse::<i32>().unwrap();
        }


        // need to count how many times a number goes past a multiple of 100
        // 50 - 68 = -18, 1 count, return to 82
        // 82 - 30 = 52, 0 count, keep at 52
        // 52 + 48 = 100, 1 count, return to 0
        // 0 - 5 = -5, 0 count, return to 95
        // 95 + 60 = 155, 1 count, return to 55
        // 55 - 55 = 0, 0 count RISK?, keep
        // 0 - 1 = -99, 1 ex count, return to 99
        // 99 - 99 = 0, 0 count RISK?, keep
        // 0 + 14 = 14 = 0 count PROBLEM, keep
        // 14 - 82 = -68, 1 ex count, return to 32

        // if i start on 0 it shouldnt count
        // if i end on 0 it should count?

        let mut hits =   (safe_value/100).abs();
        // println!("counted {hits} hits");
        print!("after spin {safe_value}, ");
        if safe_value < 0{
            safe_value += 100 * hits
        }

        if safe_value < 0{
            hits += 1;
            safe_value += 100;
            // safe_value += 100 * hits;
        }
        else if safe_value >= 100{
            safe_value -= 100 * hits;
        }

        if safe_value == 0{
            hits += 1;
        }
        zero_hits += hits;
        println!("end {safe_value}");
        println!("hits: {hits}");
        println!("total hits: {zero_hits}");

        // if safe_value == 0{
        //     // take one away because its going to get counted next loop
        //     zero_hits += 1;
        // }

    }
    // if safe_value == 0{
    //     //unless if its the last one we give it back
    //     zero_hits += 1;
    // }

    let true_safe_value = safe_value%100;
    println!("Final safe value: {true_safe_value}"); //this count seems to be consistently 49
    println!("Total zero passes: {zero_hits}")

    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");    
    // println!("With text:\n{contents}");
}

// tried solutions: 6814 +, 6813 +, 6664 -, 6706 ?

