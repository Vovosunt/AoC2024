use common::append_output_to_file;
use common::time_it;
use std::fs;
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

const DATA_PATH: &str = "data/5.txt";
const PROG_PATH: &str = "a5/src/main.rs";

fn main() -> io::Result<()> {

    let result = time_it(solution);
    append_output_to_file(PROG_PATH, format!("Solution 1: {} time: {}", result.0, result.1))?;

    Ok(())
}

fn solution() -> i32{
    if let Ok(content) = fs::read_to_string(DATA_PATH){
        let mut lines = content.lines();
        let mut order_map:HashMap<i32, HashSet<i32>> = HashMap::new();
        //parse order
        while let Some(line) = lines.next(){
            if line.is_empty(){
                break;
            }
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                if let (Ok(n1), Ok(n2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    order_map.entry(n2).or_insert(HashSet::new()).insert(n1);
                }
            }
        }
        let mut result = 0;
        //parse manuals
        while let Some(line) = lines.next(){
            let parts: Vec<i32> = line.split(',').filter_map(|x| x.parse::<i32>().ok()).collect();
            //check it
            let mut failed = false;
            for i in 0..parts.len(){
                if let Some(order_set) = order_map.get(&parts[i]){
                    let range = &parts[i..parts.len()];
                    if !range.iter().all(|&x| !order_set.contains(&x)){
                        failed = true;
                        break;
                    }
                }
            }
            if !failed{
                result += parts[(parts.len() - 1)/ 2];
            }
        }
        return result;
    }
    0
}
//Output: Solution 1: 6242 time: 636