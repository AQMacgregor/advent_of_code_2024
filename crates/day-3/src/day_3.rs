use std::fs;
use regex::Regex;

pub struct Day3{
}

impl Day3{
    pub fn new() -> Day3{
        Day3{}
    }

    pub fn part1(&mut self) -> i32{
        let contents = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-3/src/day_3_input")
        .expect("Failed to read file");
    
        let mut result =0;
        let regex = Regex::new(r"mul\(([0-9]{0,3}),([0-9]{0,3})\)").unwrap();
        for memory_line in contents.lines(){
            for (_, [first, second]) in regex.captures_iter(memory_line).map(|c| c.extract()) {
                let first = first.parse::<i32>().unwrap();
                let second = second.parse::<i32>().unwrap();
                result += first*second
            }
        }
        return result
    }

    pub fn part2(&mut self) -> i32{
        let contents = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-3/src/day_3_input")
        .expect("Failed to read file");
    
        let mut result =0;
        let regex = Regex::new(r"mul\(([0-9]{0,3}),([0-9]{0,3})\)|(don\'t)\(\)()|(do)\(\)()").unwrap();
        
        let mut turned_on = true;
        for memory_line in contents.lines(){
            for (_, [first, second]) in regex.captures_iter(memory_line).map(|c| c.extract()) {
                if first == "don't" {
                    turned_on = false;
                }else if first == "do" {
                    turned_on = true;
                }else if turned_on {
                    let first = first.parse::<i32>().unwrap();
                    let second = second.parse::<i32>().unwrap();
                    result = result + first * second;
                }
            }
        }
        return result
    }
}