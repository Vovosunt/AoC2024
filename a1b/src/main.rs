use common::append_output_to_file;
use common::time_it;
use regex::Regex;
use std::fs;
use std::io;
use std::collections::HashMap;

const DATA_PATH: &str = "data/1.txt";
const PROG_PATH: &str = "a1b/src/main.rs";

fn main() -> io::Result<()> {
    let result1 = time_it(solution1);
    let result2 = time_it(solution2);
    let result3 = time_it(solution3);
    append_output_to_file(
        PROG_PATH,
        format!("Solution 1: {} time: {} | Solution 2: {} time: {} | Solution 3: {} time: {}", 
        result1.0.to_string(), result1.1.to_string(), 
        result2.0.to_string(), result2.1.to_string(),
        result3.0.to_string(), result3.1.to_string()),
    )?;

    Ok(())
}

fn solution1() -> i32 {
    if let Ok(content) = fs::read_to_string(DATA_PATH) {
        let lines: Vec<&str> = content.lines().collect();
        let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
        let mut nums1: Vec<i32> = Vec::new();
        let mut nums2: Vec<i32> = Vec::new();
        for line in lines {
            if let Some(caps) = re.captures(&line) {
                if let (Some(g1), Some(g2)) = (caps.get(1), caps.get(2)) {
                    let n1 = g1.as_str().parse::<i32>().ok().unwrap();
                    let n2 = g2.as_str().parse::<i32>().ok().unwrap();
                    nums1.push(n1);
                    nums2.push(n2);
                }
            }
        }

        let result: i32 = nums1
            .iter()
            .map(|val| val * nums2.iter().filter(|&&x| x == *val).count() as i32)
            .sum();
        return result;
    } else {
        return 0;
    }
}

fn solution2() -> i32 {
    if let Ok(content) = fs::read_to_string(DATA_PATH) {
        let lines: Vec<&str> = content.lines().collect();
        let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
        let mut nums1: Vec<i32> = Vec::new();
        let mut nums2: Vec<i32> = Vec::new();
        for line in lines {
            if let Some(caps) = re.captures(&line) {
                if let (Some(g1), Some(g2)) = (caps.get(1), caps.get(2)) {
                    let n1 = g1.as_str().parse::<i32>().ok().unwrap();
                    let n2 = g2.as_str().parse::<i32>().ok().unwrap();
                    nums1.push(n1);
                    nums2.push(n2);
                }
            }
        }

        nums1.sort();
        nums2.sort();
        
        let mut counter = 0;
        let mut done = false;
        let mut result = 0;
        for x in nums1.iter(){
            if done{
            } else{
                while *x > nums2[counter]{
                    counter += 1;
                    if !(counter < nums2.len()){
                        done = true;
                        break;
                    }
                }
                if done{
                    break;
                }
                let mut total = 0;
                while *x == nums2[counter]{
                    counter += 1;
                    if !(counter < nums2.len()){
                        done = true;
                        break;
                    }
                    total += 1;
                }
                result += x * total;
            }
        }

        return result;
    } else {
        return 0;
    }
}

fn solution3() -> i32 {
    if let Ok(content) = fs::read_to_string(DATA_PATH) {
        let mut nums1: Vec<i32> = Vec::new();
        let mut nums2: Vec<i32> = Vec::new();

        //manual parsing
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(n1), Ok(n2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    nums1.push(n1);
                    nums2.push(n2);
                }
            }
        }

        //Count with HashMap
        let mut count_map = HashMap::new();
        for &num in &nums2 {
            *count_map.entry(num).or_insert(0) += 1;
        }

        let mut result = 0;
        for x in &nums1{
            result += x * count_map.get(x).unwrap_or(&0);
        }
        
        return result;
    } else {
        return 0;
    }
}
//Output: Solution 1: 24931009 time: 1556 | Solution 2: 24931009 time: 921 | Solution 3: 24931009 time: 226