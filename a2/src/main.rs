use regex::Regex;
use std::fs;
use std::io;
use common::append_output_to_file;

fn check_nums(nums: &Vec<i32>) -> bool {
    let direction = nums[0] < nums[1];
    for i in 1..nums.len() {
        if direction != (nums[i - 1] < nums[i]) {
            return false;
        }
        let diff = (nums[i - 1] - nums[i]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn main() -> io::Result<()> {
    let data_path = "data/2.txt";
    let prog_path = "a2/src/main.rs";
    let content = fs::read_to_string(data_path)?;
    let lines: Vec<&str> = content.lines().collect();
    let re = Regex::new(r"(\d+)").unwrap();
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let line_nums: Vec<i32> = re
            .captures_iter(&line)
            .map(|cap| cap[0].parse::<i32>().ok().unwrap())
            .collect();
        numbers.push(line_nums);
    }

    // for line in numbers.iter(){
    //     println!("{:?} : {:?}", check_nums(line), line);
    // }

    let result: Vec<_> = numbers.iter().filter(|&line| check_nums(line)).collect();
    println!("{}", result.len());

    append_output_to_file(prog_path, result.len().to_string())?;

    Ok(())
}
//Output: 356