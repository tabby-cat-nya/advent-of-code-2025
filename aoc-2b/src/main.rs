// within certain rnages, find numbers with a pattern repeated exactly twice
// eg "55" ("5" twice), "6464" ("64" twice), "123123"
// cannot be matches on numebrs with an odd number of digits
// compare the first half of the didgits to the second half
// save all the matches (invalid id's) to a list and add them up to produce the solution
// each rnage can have 0 or more invalid id's
// need to parse each range (probably regex), get start and end value for each 

use std::fs;
use regex::Regex;

fn main() {
    println!("Lets begin, day 2, part 1");
    let file_path = "src/input.txt";
    // let file_path = "src/example.txt";

    // split up input by "," and "-"
    // (?<start_range>\d*)-(?<end_range>\d*)
    let re = Regex::new(r"(?<start_range>\d*)-(?<end_range>\d*)").unwrap();
    let line = fs::read_to_string(file_path).unwrap();
    println!("{line}");

    let mut invalids: Vec<i64> = Vec::new();

    // parse the provided ranges into paits
    let ranges: Vec<(&str, &str)> = re.captures_iter(&line).map(|caps| {
        let start = caps.name("start_range").unwrap().as_str();
        let end = caps.name("end_range").unwrap().as_str();
        (start, end)
    }).collect();

    println!("{ranges:?}");
    for range in ranges{
        println!("working on: {range:?}");
        let start = range.0.parse::<i64>().unwrap();
        let end = range.1.parse::<i64>().unwrap();
        // println!("{range.0:?} --> {end}");
        let mut count :i64 = start;
        while count <= end{
            // if odd number of didgits, then no match, skip to next loop
            // if even number, then compare the first and second half
            let current = count.to_string();
            
            if find_invalid(current){
                invalids.push(count);
            }

            // if current.len() % 2 != 1{
            //     let half_len = current.len()/2;
            //     let half_a = &current[..half_len];
            //     let half_b = &current[half_len..];

            //     let current_str = current.to_string();
            //     println!("{current_str} = {half_a} + {half_b}");

            //     if half_a == half_b{
            //         invalids.push(count);
            //     }
            // }
            count += 1;
        }
    }

    println!("{invalids:?}");
    let mut solution: i64 = 0;
    for result in invalids{
        solution += result;
    }
    println!("Puzzle Solution: {solution}")
    // Correct :3

}

fn find_invalid(text : String) -> bool{
    // let mut invalid = false;
    // 1x matches
    if text.len() > 1{
        let one = text.chars().nth(0).unwrap();
        let mut ones_match = true;
        for char in text.chars(){
            if char == one{
                // good
            }
            else{
                ones_match = false;
            }
        }
        if ones_match{
            return true;
        }
    }
    

    // 2x matches
    if text.len() == 4{
        if &text[0..2] == &text[2..4]{
            return true;
        }
    }
    else if text.len() == 6{
        if &text[0..2] == &text[2..4] && &text[0..2] == &text[4..6]{
            return true;
        }
    }
    else if text.len() == 8{
        if &text[0..2] == &text[2..4] && &text[0..2] == &text[4..6] && &text[0..2] == &text[6..8]{
            return true;
        }
    }
    else if text.len() == 10{
        if &text[0..2] == &text[2..4] && &text[0..2] == &text[4..6] && &text[0..2] == &text[6..8] && &text[0..2] == &text[8..10]{
            return true;
        }
    }
    

    // 3x matches
    if text.len() == 6 {
        if &text[0..3] == &text[3..6]{
            return true;
        }
    }
    if text.len() == 9 {
        if &text[0..3] == &text[3..6] && &text[0..3] == &text[6..9]{
            return true;
        }
    }

    // 4x matches
    if text.len() == 8 {
        if &text[0..4] == &text[4..8]{
            return true;
        }
    }

    // 5x matches
    if text.len() == 10{
        if &text[0..5] == &text[5..10]{
            return true;
        }
    }


    return false; // represents no match
}

// tried: 36862281460 - too high
// solved! second attempt