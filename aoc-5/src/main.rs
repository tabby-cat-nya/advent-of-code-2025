use std::fs;
use regex::Regex;
fn main() {
    println!("Lets begin, day 5");
    // let file_path = "src/input.txt";
    let file_path = "src/example.txt";

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

    // println!("Fresh Products: {fresh:?}");
    // println!("Spoiled Products: {spoiled:?}");
    let fresh_count = fresh.len();
    let spoiled_count = spoiled.len();
    println!("Fresh Count: {fresh_count:?}");
    println!("Spoiled Count: {spoiled_count:?}");

    // part 2 logic - how many fresh id's are there in total

    // brute force will take literally forever
    // sort all the ranges based on their start number
    // then go through all of them and if start is less than the last ranges end make it one more than the last ranges end
    // then calculate the difference between each range and add them up for each range (end - start + 1)

    let mut all_fresh: Vec<i64> = Vec::new();
    // for range in &ranges{
    //     for id in range.0..=range.1 {
    //         if !all_fresh.contains(&id) {
    //             all_fresh.push(id);
    //         }
    //     }
    //     print!(".");
    // }
    
    //sorting time
    println!("{ranges:?}");
    ranges.sort_by(|a,b| a.0.cmp(&b.0));
    println!("{ranges:?}");

    let mut last_end = -1; //? 
    for mut range in &mut ranges{
        if range.0 < last_end{
            range.0 = last_end + 1;
        }
        last_end = range.1;
    }
    
    // let meow = &ranges;
    println!("{ranges:?}");
    let mut all_count:i64 = 0;
    for mut range in &mut ranges{
        if range.0 > range.1 {
            // invalid range, disregard
        }
        else {
            all_count += range.1 - range.0 + 1;
        }
        
    }
    
    // let all_count = all_fresh.len();
    println!("All fresh id count: {all_count:?}");
    
    // too low:     330684820916269
    // too high:    348115621205555
    

}
