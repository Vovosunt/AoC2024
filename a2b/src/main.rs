use regex::Regex;
use std::fs;
use std::io;
use common::append_output_to_file;

fn check_nums(nums: &Vec<i32>, did_skip: bool) -> bool {
    let direction = nums[0] < nums[1];
    let mut skipped = 0;
    for i in 1..nums.len() {
        if direction != (nums[i - 1] < nums[i]) {
            if did_skip {
                return false;
            }
            skipped = i;
            break;
        }
        let diff = (nums[i - 1] - nums[i]).abs();
        if diff < 1 || diff > 3 {
            if did_skip {
                return false;
            }
            skipped = i;
            break;
        }
    }
    if skipped != 0 {
        // let mut v1 = nums.clone();
        // v1.remove(skipped);
        // println!("{:?}",v1);
        // let mut v2 = nums.clone();
        // v2.remove(skipped - 1);
        // println!("{:?}",v2);
        // return check_nums(&v1, true) || check_nums(&v2, true);
        for i in 0..nums.len() {
            let mut v = nums.clone();
            v.remove(i);
            if check_nums(&v, true) {
                return true;
            }
        }
        return false;
    }
    return true;
}

fn main() -> io::Result<()> {
    let data_path = "data/2.txt";
    let prog_path = "a2b/src/main.rs";
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

    let result: Vec<_> = numbers
        .iter()
        .filter(|&line| check_nums(line, false))
        .collect();
    println!("{}", result.len());

    append_output_to_file(prog_path, result.len().to_string())?;

    Ok(())
}
//Output: 413