use std::{cmp::Ordering, fs};

pub struct Day5{
    rules: Vec<Rule>
}

pub struct Rule{
    left: String,
    right: String
}

impl Day5{
    pub fn new() -> Day5{
        Day5{
            rules: Vec::new()
        }
    }

    pub fn part1(&mut self) -> i32{
        let rules = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-5/src/day_5_rules_input")
        .expect("Failed to read file");

        for rule in rules.lines(){
            let parts: Vec<&str> = rule.split("|").collect();
            let left = parts[0].to_string();
            let right = parts[1].to_string();
            self.rules.push(Rule::new(left, right));
        }
        
        let pages = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-5/src/day_5_pages_input")
            .expect("Failed to read file");
        
        let mut result = 0;
        for page in pages.lines(){
            let parts: Vec<&str> = page.split(",").collect();
            let mut is_valid = true;
            let middle_index = (parts.len()- 1)/2;
            let middle = parts[middle_index].parse::<i32>().unwrap();
            for rule in self.rules.iter(){
                if !rule.validate(parts.iter().map(|p| p.to_string()).collect()){
                    is_valid = false;
                }
            }
            if is_valid {
                result += middle;
            }
        }


        return result
    }

    pub fn part2(&mut self) -> i32{
        let rules = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-5/src/day_5_rules_input")
            .expect("Failed to read file");

        for rule in rules.lines(){
            let parts: Vec<&str> = rule.split("|").collect();
            let left = parts[0].to_string();
            let right = parts[1].to_string();
            self.rules.push(Rule::new(left, right));
        }
        
        let pages = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-5/src/day_5_pages_input")
            .expect("Failed to read file");
        
        let mut result = 0;
        for page in pages.lines(){
            let mut parts: Vec<String> = page.split(",").map(|p| p.to_string()).collect();
            let mut is_valid = true;
            for rule in self.rules.iter(){
                if !rule.validate(parts.iter().map(|p| p.to_string()).collect()){
                    is_valid = false;
                }
            }
            if is_valid{
               continue; 
            }
            parts.sort_by(|a, b| self.sort(a, b));

            let middle_index = (parts.len()- 1)/2;
            let middle = parts[middle_index].parse::<i32>().unwrap();
            result += middle;
        }


        return result
    }
    
    fn sort (&mut self, a: &String, b:&String) -> Ordering{
        let mut page_part = Vec::new();
        page_part.push(a.to_string());
        page_part.push(b.to_string());
        for rule in self.rules.iter() {
            if !rule.validate(page_part.clone()){
                return Ordering::Greater
            }
        }
        return Ordering::Less;
    }
}

impl Rule {
    pub fn new(left: String, right: String) -> Rule{
        Rule{
            left,
            right
        }
    }

    pub fn validate(&self, page: Vec<String>) -> bool{
        let mut left_index: Option<usize> = None;
        let mut right_index: Option<usize> = None;
        for (i, value) in page.iter().enumerate() {
            if *value == self.left {
                left_index = Some(i)
            }
            if *value == self.right {
                right_index = Some(i)
            }
        }
        if left_index.is_none() || right_index.is_none(){
            return true;
        }
        let l = left_index.unwrap();
        let r = right_index.unwrap();
        if l < r {
            return true
        }
        return false
    }
}