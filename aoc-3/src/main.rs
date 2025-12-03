
use std::fs;

fn main() {
    println!("Lets begin, day 2, both parts!");
    let file_path = "src/input.txt";
    // let file_path = "src/example.txt";

    let mut total_joltage = 0;
    let mut count = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        // total_joltage += _find_highest_joltage(line);
        count += 1;
        print!("checking bank {count}... ");
        total_joltage += _find_highest_joltage_part2(line);
        
    }
    println!("The total joltage of all {count:?} banks is: {total_joltage}")
}

fn _find_highest_joltage(line : &str) -> u32{
    // comb through to find the highest 2 numbres without breaking ordering
    let mut highest_joltage = 0;
    let line_size = line.chars().count();
    

    // for char in line.chars(){
    //     println!("{char}");
    // }
    for a in 0..line_size{
        for b in a+1..line_size{
            let a_value = line.chars().nth(a).unwrap().to_digit(10).unwrap();
            let b_value = line.chars().nth(b).unwrap().to_digit(10).unwrap();
            let joltage = (10 * a_value) + b_value;
            // println!("{a_value:?} + {b_value:?} = {a_value:?}{b_value:?} should match {joltage:?}");
            if joltage > highest_joltage{
                highest_joltage = joltage
            }

            
        }
    }
    println!("This banks best joltage is: {highest_joltage}");
    return highest_joltage;
}


fn _find_highest_joltage_part2(line : &str) -> u64{
    // should return the highest battery vlaue found in the bank
    let mut joltage_string:String = "".to_owned();

    // "take earliest index of highest possible digit until a valid solution is found"
    // need to keep track of last possible index selectable, start index is last one taken
    let mut start_index = 0;
    // let mut end_buffer = 11;
    let line_size = line.chars().count();


    for i in 0..12{ // for each character in the joltage string
        let mut biggest = 0;
        let mut biggest_index = 0;
        let mut view_search : String = "".to_owned();
        for x in start_index..line_size-(11-i){ //take earliest index of highest possible digit
            view_search += &line.chars().nth(x).unwrap().to_string();
            if line.chars().nth(x).unwrap().to_digit(10).unwrap() > biggest{
                biggest = line.chars().nth(x).unwrap().to_digit(10).unwrap();
                biggest_index = x+1;
            }
        }
        // println!("debug view - {view_search}");
        joltage_string += &biggest.to_string();
        start_index = biggest_index;
        // end_buffer -= 1;
        
        
    }
    
    // println!("{joltage_string}");
    let joltage = joltage_string.parse::<u64>().unwrap();
    println!("This banks best joltage is: {joltage:?}");
    return joltage;
}

