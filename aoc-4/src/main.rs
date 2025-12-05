use std::fs;
use grid::*;
fn main() {
    println!("Lets begin, day 4, part 1");
    // let file_path = "src/input.txt";
    let file_path = "src/example.txt";

    //figure out size
    let mut x_size = 0;
    let mut y_size = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        // create some sort of grd structure to hold all the symbols
        // also need to figure out how to deal with the edges 

        if x_size == 0{
            for char in line.chars(){
                x_size += 1
            }
        }
        
        y_size += 1
        
        
    }
    println!("Grid Size: {x_size} x {y_size}");

    let mut grid : Grid<char> = Grid::new(y_size,x_size);
    
    
    let mut x = 0;
    let mut y = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        // create some sort of grd structure to hold all the symbols
        // also need to figure out how to deal with the edges 

        for char in line.chars(){
            grid[(y,x)] = char;
            let value = grid.get(x,y).unwrap();
            println!("{value:?}");
            x += 1;
        }
        y+=1;
        x=0;
        
    }

    println!("{grid:?}")
}
