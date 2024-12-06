use regex::Regex;
use std::fs;
use std::io;
use common::append_output_to_file;

fn main() -> io::Result<()> {
    let data_path = "data/3.txt";
    let prog_path = "a3/src/main.rs";
    let content = fs::read_to_string(data_path)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let numbers: Vec<(i32,i32)> = re
        .captures_iter(&content)
        .filter_map(|cap| {
            let n1 = cap[1].parse::<i32>().ok()?;
            let n2 = cap[2].parse::<i32>().ok()?;
            Some((n1,n2))
        })
        .collect();

    let result: i32 = numbers.iter().map(|&x| x.0 * x.1).sum();

    append_output_to_file(prog_path, result.to_string())?;

    Ok(())
}
//Output: 188741603