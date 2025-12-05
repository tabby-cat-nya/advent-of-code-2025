use std::fs;
use grid::*;
fn main() {
    println!("Lets begin, day 4, part 2");
    let file_path = "src/input.txt";
    // let file_path = "src/example.txt";

    //figure out size
    let mut x_size: i32 = 0;
    let mut y_size: i32 = 0;
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

    let mut grid : Grid<char> = Grid::new(y_size as usize,x_size as usize);
    
    
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

    // println!("{grid:#?}");

    // now determine which ones can be removed 
    let mut total_removed = 0;
    let mut before_action = 0;
    let mut after_action = 0;
    let mut changed_grid : Grid<char> = Grid::new(0,0);
    let mut removed = access_check(grid, x_size, y_size, &mut changed_grid);
    after_action = removed;
    total_removed += removed;
    while after_action > before_action{
        before_action = after_action;
        // run it again, add to total
        let removed_more = access_check(changed_grid.clone(), x_size, y_size, &mut changed_grid);
        after_action += removed_more;
        total_removed += removed_more;
        // println!("total removed so far: {total_removed}")
    }

    // let result = access_check(grid, x_size, y_size);
    println!("Output: {total_removed} rolls are accessible");

    // println!("{changed_grid:#?}");

    

}

fn access_check(mut grid : Grid<char>, x_size : i32, y_size : i32, changed_grid : &mut Grid<char>) -> i32 {
    let mut accessible  = 0;
    let mut demo_grid = grid.clone();

    
    for x in 0..x_size {
        // create some sort of grd structure to hold all the symbols
        // also need to figure out how to deal with the edges 

        for y in 0..y_size {
            // grid[(y,x)] = char;
            // let value = grid.get(x,y).unwrap();
            // println!("{value:?}");
            // x += 1;
            if grid[(y as usize ,x as usize)]  == '@'{
                //meow
                let mut rolls = 0;
                rolls += check_tile(x-1, y-1, &grid, x_size, y_size);
                rolls += check_tile(x-1, y, &grid, x_size, y_size);
                rolls += check_tile(x-1, y+1, &grid, x_size, y_size);
                rolls += check_tile(x, y-1, &grid, x_size, y_size);
                rolls += check_tile(x, y+1, &grid, x_size, y_size);
                rolls += check_tile(x+1, y-1, &grid, x_size, y_size);
                rolls += check_tile(x+1, y, &grid, x_size, y_size);
                rolls += check_tile(x+1, y+1, &grid, x_size, y_size);
                if rolls < 4{
                    accessible += 1;
                    demo_grid[(y as usize ,x as usize)]  = 'X';
                }
            }
            // println!(grid[(y)]);
            
        }
        
        
    }
    // println!("{demo_grid:#?}");
    println!("acessible this pass: {accessible}");
    *changed_grid = demo_grid;
    

    return accessible;
}

fn check_tile(x : i32, y:i32, grid : &Grid<char>, x_size : i32, y_size : i32) -> i32{
    // returns 1 if the specified tile contains an '@'
    // otherwise returns 0, including if the tile is out of bounds
    if x < 0 || x > x_size-1{
        return 0;
    }
    else if y < 0 || y > y_size-1 {
        return 0;
    }

    if grid[(y as usize,x as usize)] == '@'{
        return 1;
    }
    else{
        return 0;
    }

    // return 0;
}

