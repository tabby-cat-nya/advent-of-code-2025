
use std::fs;
fn main() {
    println!("Lets begin, day 6");
    let file_path = "src/input.txt";
    // let file_path = "src/example.txt";

    // split innto a vector of string
    // figure out how many questions there are len()/4 -> q
    // for each question we have a set of indexes eg: 0+i,q*1+i,q*2+i,q*3+i - q*3 is the operator 

    let mut line_count = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        line_count += 1;
    }
    println!("Lines: {line_count:?}");

    let line = fs::read_to_string(file_path).unwrap();
    // println!("{line}");
    let mut elements: Vec<String> = Vec::new();
    for element in line.split_whitespace(){
        elements.push(element.to_string());
    }
    println!("{elements:?}");
    let ele_size = elements.len();
    println!("amount: {ele_size:?}");

    
    let questions : usize = elements.len() / line_count;
    println!("Number of questions to solve: {questions:?}");

    let mut i = 0;
    let mut grand_total = 0;
    while i < questions{
        let operator = elements.get(((questions)*(line_count-1))+i).unwrap();
        // println!("{operator:?}");
        if operator == "+" {
            let mut result = 0;
            let mut x = 0;
            while x < line_count-1 {
                // println!("x - {x}");
                // let accessing = elements.get((questions*x)+i).unwrap();
                // println!("{accessing:?}");
                result += elements.get((questions*x)+i).unwrap().parse::<i64>().unwrap();
                x += 1;
            }
            grand_total += result;
            println!("Question {i:?}, solution: {result:?}");
        }
        else if operator == "*" {
            let mut result= elements.get((questions*0)+i).unwrap().parse::<i64>().unwrap();
            let mut x = 1;
            while x < line_count-1 {
                // println!("x - {x}");
                result *= elements.get((questions*x)+i).unwrap().parse::<i64>().unwrap();
                x += 1;
            }
            grand_total += result;
            println!("Question {i:?}, solution: {result:?}");
        }
        else {
            println!("womp womp -> {operator:?}");
        }
        
        i += 1;
    }

    println!("Grand Total: {grand_total:?}")

}