use std::fs;
use std::io::{self, Write};
use std::time::Instant;

pub fn append_output_to_file(file_path: &str, output: String) -> io::Result<()> {
    //Append output to this file
    let prog_content = fs::read_to_string(file_path)?;
    let prog_lines: Vec<&str> = prog_content.lines().collect();
    let mut new_prog_lines = prog_lines[0..prog_lines.len() - 1].join("\n");

    if let Some(last_line) = prog_lines.last() {
        if !last_line.starts_with("//") {
            new_prog_lines.push_str("\n");
            new_prog_lines.push_str(last_line);
        }
    }

    new_prog_lines.push_str(&format!("\n//Output: {}", output));

    let mut file: fs::File = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(new_prog_lines.as_bytes())?;

    Ok(())
}

pub fn time_it<F, U>(func: F) -> (U, u128)
where
    F: Fn() -> U,
{
    let start = Instant::now();
    let result = func();
    let elapsed = Instant::now() - start;
    return (result, elapsed.as_micros());
}

pub fn get_char_grid(input: &str) -> Vec<Vec<char>>{
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines(){
        grid.push(line.chars().collect());
    }
    return grid;
}
