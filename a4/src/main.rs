use common::append_output_to_file;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let data_path = "data/4.txt";
    let prog_path = "a4/src/main.rs";

    let content = fs::read_to_string(data_path)?;
    let lines: Vec<&str> = content.lines().collect();

    const DIRS: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
    let size: (i32, i32) = (lines[0].len() as i32, lines.len() as i32);

    let mut grid: Vec<Vec<char>> = Vec::with_capacity(size.1 as usize);

    for line in lines.iter() {
        grid.push(line.chars().collect());
    }

    let mut result: i32 = 0;

    for x in 0..size.0 {
        for y in 0..size.1 {
            if grid[y as usize][x as usize] == 'X' {
                for dir in DIRS {
                    let mut failed = false;
                    for i in 1..4 {
                        let val = grid
                            .get((y + dir.1 * i) as usize)
                            .and_then(|row| row.get((x + dir.0 * i) as usize))
                            .unwrap_or(&'0');
                        if *val != WORD[i as usize] {
                            failed = true;
                            break;
                        }
                    }
                    if !failed {
                        result += 1;
                    }
                }
            }
        }
    }

    append_output_to_file(prog_path, result.to_string())?;

    Ok(())
}
//Output: 2462