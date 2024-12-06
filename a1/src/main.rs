use common::append_output_to_file;
use regex::Regex;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let data_path = "data/1.txt";
    let prog_path = "a1/src/main.rs";
    let content = fs::read_to_string(data_path)?;
    let lines: Vec<&str> = content.lines().collect();
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();
    for line in lines {
        if let Some(caps) = re.captures(&line){
                if let (Some(g1),Some(g2)) = (caps.get(1), caps.get(2)){
                let n1 = g1.as_str().parse::<i32>().ok().unwrap();
                let n2 = g2.as_str().parse::<i32>().ok().unwrap();
                nums1.push(n1);
                nums2.push(n2);
            }
        }
    }

    nums1.sort();
    nums2.sort();

    let result: i32 = nums1
        .iter()
        .zip(nums2.iter())
        .map(|val| (val.0 - val.1).abs())
        .sum();

    append_output_to_file(prog_path, result.to_string())?;

    Ok(())
}
//Output: 2066446