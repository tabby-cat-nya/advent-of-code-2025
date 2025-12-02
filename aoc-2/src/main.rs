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

    // let mut results = vec![];

    // parse the provided ranges into paits
    let ranges: Vec<(i32, i32)> = re.captures_iter(&line).map(|caps| {
        let start: i32 = caps.name("start_range").unwrap().as_str().parse::<i32>().unwrap();
        let end: i32 = caps.name("end_range").unwrap().as_str().parse::<i32>().unwrap();
        (start, end)
    }).collect();

    println!("{ranges:?}");


}
