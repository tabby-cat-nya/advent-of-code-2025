
use std::fs;

fn main() {
    println!("Lets begin, day 2, part 1");
    // let file_path = "src/input.txt";
    let file_path = "src/example.txt";

    let mut total_joltage = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        total_joltage += find_highest_joltage(line);
    }
    println!("The total joltage is: {total_joltage}")
}

fn find_highest_joltage(line : &str) -> u32{
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

// fn find_highest_joltage_recursive(line : &str, start : u32, end : u32, levels : u32) -> u32{
//     for i in start+1..end{

//     }
    
//     return 1;
// }


fn get_biggest_digit(line : &str, start : usize) -> u32{
    let mut biggest = 0;
    let line_size = line.chars().count();
    
    for i in start..line_size{
        if line.chars().nth(i).unwrap().to_digit(10).unwrap() > biggest{
            biggest = line.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
    }
    return biggest;
}

