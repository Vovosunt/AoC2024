use regex::Regex;
use common::append_output_to_file;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let data_path = "data/3.txt";
    let prog_path = "a3b/src/main.rs";
    let content = fs::read_to_string(data_path)?;
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d+),(\d+)\))").unwrap();
    let mut result: i32 = 0;
    let mut skip = false;
    for caps in re.captures_iter(&content) {
        if let Some(_) = caps.get(1) {
            //do match
            skip = false;
        }
        if let Some(_) = caps.get(2) {
            //don't match
            skip = true;
        }

        if !skip {
            //mul match
            if let (Some(first_num), Some(second_num)) = (caps.get(4), caps.get(5)) {
                if let (Ok(first), Ok(second)) = (
                    first_num.as_str().parse::<i32>(),
                    second_num.as_str().parse::<i32>(),
                ) {
                    result += first * second;
                }
            }
        }
    }

    append_output_to_file(prog_path, result.to_string())?;

    Ok(())
}
//Output: 67269798