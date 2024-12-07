use common::append_output_to_file;
use common::get_char_grid;
use common::time_it;
use std::collections::HashSet;
use std::fs;
use std::io;

const DATA_PATH: &str = "data/6.txt";
const PROG_PATH: &str = "a6b/src/main.rs";

fn main() -> io::Result<()> {
    let result = time_it(solution1);
    append_output_to_file(
        PROG_PATH,
        format!("Solution 1: {} time: {}", result.0, result.1),
    )?;

    Ok(())
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn solution1() -> i32 {
    if let Ok(content) = fs::read_to_string(DATA_PATH) {
        let grid = get_char_grid(&content);
        let size = (grid.len() as i32, grid[0].len() as i32); //y,x
        let mut pos: (i32, i32) = (0, 0);
        let mut current_dir = 0; //facing up at the start

        for y in 0..size.0 as usize {
            for x in 0..size.1 as usize {
                if grid[y][x] == '^' {
                    //get start
                    pos = (y as i32, x as i32);
                    break;
                }
            }
        }
        //checks if position is inside bounds
        let in_bounds =
            |pos: (i32, i32)| pos.0 >= 0 && pos.0 < size.0 && pos.1 >= 0 && pos.1 < size.1;

        let mut initial_path: HashSet<(usize, (i32, i32))> = HashSet::new();
        let mut hits = HashSet::new();
        while in_bounds(pos) {
            if grid[pos.0 as usize][pos.1 as usize] == '#' {
                //test collision and redirect
                pos.0 -= DIRS[current_dir].0;
                pos.1 -= DIRS[current_dir].1;
                current_dir = (current_dir + 1) % DIRS.len();
            } else if !initial_path.contains(&(current_dir, pos)){
                //add to path
                initial_path.insert((current_dir, pos));
                //test if next is not obstacle already and is in bounds first
                let tmp_pos = (pos.0 + DIRS[current_dir].0, pos.1 + DIRS[current_dir].1);

                if in_bounds(tmp_pos) && grid[tmp_pos.0 as usize][tmp_pos.1 as usize] != '#' {
                    //test for loops if obstacle would be next
                    let mut path: HashSet<(usize, (i32, i32))> = initial_path.clone();
                    let mut path_dir: usize = current_dir;
                    let mut path_pos = pos;
                    //manually turn to simulate obstacle
                    path_dir = (path_dir + 1) % DIRS.len();
                    while in_bounds(path_pos) {
                        if grid[path_pos.0 as usize][path_pos.1 as usize] == '#' {
                            //test collision and redirect
                            path_pos.0 -= DIRS[path_dir].0;
                            path_pos.1 -= DIRS[path_dir].1;
                            path_dir = (path_dir + 1) % DIRS.len();
                        } else {
                            let mark = (path_dir, path_pos);
                            if path.contains(&mark) {
                                //avoid duplicate obstacles placed from different sides
                                hits.insert(tmp_pos);
                                break;
                            } else {
                                path.insert(mark);
                            }
                        }
                        path_pos.0 += DIRS[path_dir].0;
                        path_pos.1 += DIRS[path_dir].1;
                    }
                }
            }
            //move
            pos.0 += DIRS[current_dir].0;
            pos.1 += DIRS[current_dir].1;
        }
        let result = hits.len() as i32;
        return result;
    }
    0
}

// else if grid[y][x] == '#' {
//     //save jumps to obstacles
//     for dir in 0..DIRS.len() {
//         let incoming_dir = (dir + 1) % DIRS.len();
//         let offset = DIRS[incoming_dir];
//         let mut dir_pos = (y as i32 + DIRS[dir].0, x as i32 + DIRS[dir].1);
//         let jump_to = dir_pos;
//         while dir_pos.0 >= 0
//             && dir_pos.0 < size.0
//             && dir_pos.1 >= 0
//             && dir_pos.1 < size.1
//         {
//             if grid[dir_pos.0 as usize][dir_pos.1 as usize] == '#' {
//                 break;
//             } else {
//                 //check if the obstacle would be in bounds
//                 let mark_pos = (dir_pos.0 + offset.0, dir_pos.1 + offset.1);
//                 if !(mark_pos.0 >= 0
//                     && mark_pos.0 < size.0
//                     && mark_pos.1 >= 0
//                     && mark_pos.1 < size.1)
//                 {
//                     break;
//                 }
//             }
//             //checks passed, record jump
//             obstacles[dir_pos.0 as usize][dir_pos.1 as usize][incoming_dir] =
//                 jump_to;

//             dir_pos.0 += DIRS[dir].0;
//             dir_pos.1 += DIRS[dir].1;
//         }
//     }
// }

//Output: Solution 1: 1932 time: 484378