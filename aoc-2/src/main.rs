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

            count += 1;
        }
    }


}
