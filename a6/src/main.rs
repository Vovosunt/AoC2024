use common::append_output_to_file;
use common::get_char_grid;
use common::time_it;
use std::fs;
use std::io;

const DATA_PATH: &str = "data/6.txt";
const PROG_PATH: &str = "a6/src/main.rs";

fn main() -> io::Result<()> {
    let result = time_it(solution);
    append_output_to_file(
        PROG_PATH,
        format!("Solution 1: {} time: {}", result.0, result.1),
    )?;

    Ok(())
}

fn solution() -> i32 {
    if let Ok(content) = fs::read_to_string(DATA_PATH) {
        let mut grid = get_char_grid(&content);
        let size = (grid.len() as i32, grid[0].len() as i32); //y,x
        let mut pos: (i32, i32) = (0, 0);
        const DIRS: [(i32,i32); 4] = [(-1,0), (0,1), (1,0), (0,-1)];
        let mut current_dir = 0; //facing up at the start
        let mut result = 0;

        for y in 0..size.0 as usize {
            for x in 0..size.1 as usize{
                if grid[y][x] == '^'{
                    pos = (y as i32,x as i32);
                }
            }
        }

        while pos.0 >= 0 && pos.0 < size.0 && pos.1 >= 0 && pos.1 < size.1{
            if grid[pos.0 as usize][pos.1 as usize] == '#'{
                //test collision and redirect
                pos.0 -= DIRS[current_dir].0;
                pos.1 -= DIRS[current_dir].1;
                current_dir = (current_dir + 1) % DIRS.len();
            } else if grid[pos.0 as usize][pos.1 as usize] != '0'{
                //add and mark
                result += 1;
                grid[pos.0 as usize][pos.1 as usize] = '0';
            }
            //move
            pos.0 += DIRS[current_dir].0;
            pos.1 += DIRS[current_dir].1;
        }


        return result;
    }
    0
}
//Output: Solution 1: 5318 time: 291