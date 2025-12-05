use std::fs;
use regex::Regex;
fn main() {
    println!("Lets begin, day 5");
    let file_path = "src/input.txt";
    // let file_path = "src/example.txt";

    let re = Regex::new(r"(?<low>\d*)-(?<high>\d*)").unwrap();
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut fresh: Vec<i64> = Vec::new();
    let mut spoiled: Vec<i64> = Vec::new();
    let mut stage = 0;
    
    for line in fs::read_to_string(file_path).unwrap().lines() {
        // create some sort of grd structure to hold all the symbols
        // also need to figure out how to deal with the edges 

        println!("{line}");
        if line == "" {
            //we've reached the gap betweent he ranges and the products list
            stage = 1;
            println!("Reading in ranges complete:");
            println!("{ranges:?}");
        }

        if stage == 0{
            let mut range: Vec<(i64, i64)> = re.captures_iter(&line).map(|caps| {
                let start: i64 = caps.name("low").unwrap().as_str().parse::<i64>().unwrap();
                let end: i64 = caps.name("high").unwrap().as_str().parse::<i64>().unwrap();
                (start, end)
            }).collect();
            ranges.append(&mut range);
        }
        
        if stage == 1 && line != "" {
            let product = line.parse::<i64>().unwrap();
            let mut checked = false;
            for check_range in &ranges{
                if product >= check_range.0 && product <= check_range.1 {
                    // this one is fresh
                    fresh.push(product);
                    checked = true;
                    break;
                } 
            }
            if !checked {
                spoiled.push(product);
            }
        }
        
        
    }

    println!("Fresh Products: {fresh:?}");
    println!("Spoiled Products: {spoiled:?}");
    let fresh_count = fresh.len();
    let spoiled_count = spoiled.len();
    println!("Fresh Count: {fresh_count:?}");
    println!("Spoiled Count: {spoiled_count:?}");

    
    
    
    

}
