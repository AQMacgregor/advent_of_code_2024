use std::fs;

pub struct Day2{
}

impl Day2{
    pub fn new() -> Day2{
        Day2{}
    }

    pub fn part1(&mut self) -> i32{
        let contents = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-2/src/day_2_input")
        .expect("Failed to read file");

        let mut result = 0;
        for report in contents.lines() {
            if check_report_1(report) {
                result += 1;
            }
        }

        return result;
    }

    pub fn part2(&mut self) -> i32{
        let contents = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-2/src/day_2_input")
        .expect("Failed to read file");

        let mut total_valid = 0;
        for report in contents.lines() {
            if check_report_2(report, None) {
                total_valid += 1;
            } else {
                let mut valid = false;
                for (i, _) in report.split(" ").enumerate() {
                    if check_report_2(report, Some(i)){
                        valid = true;
                        break;
                    }
                }
                if valid {
                    total_valid += 1;
                }
            }
        }

        return total_valid;
    }
}

fn check_report_1(report: &str) -> bool {
    let mut prev_level = 0;
    let mut direction = 0;
    for level_str in report.split(" "){
        let level = level_str.parse::<i32>().unwrap();  
        if prev_level == 0{
            
        } else if prev_level == level {
            return false;
        } else {
            if direction == 0 {
                if prev_level < level {
                    direction = 1;
                }else{
                    direction = -1;
                }
            }
            if direction == 1 {
                if level < prev_level {
                    return false
                }
                if level - prev_level > 3 {
                    return false
                }
            }else{
                if prev_level < level {
                    return false
                }
                if prev_level - level > 3 {
                    return false
                }
            }
        }
        prev_level = level;
    }
    return true
}

fn check_report_2(report: &str, index_to_ignore: Option<usize>) -> bool {
    let mut prev_level = 0;
    let mut direction = 0;
    for (i, level_str) in report.split(" ").enumerate() {
        if index_to_ignore == Some(i){
            continue;
        }
        let level = level_str.parse::<i32>().unwrap();  
        if prev_level == 0{
            
        } else if prev_level == level {
            return false;
        } else {
            if direction == 0 {
                if prev_level < level {
                    direction = 1;
                }else{
                    direction = -1;
                }
            }
            if direction == 1 {
                if level < prev_level {
                    return false
                }
                if level - prev_level > 3 {
                    return false
                }
            }else{
                if prev_level < level {
                    return false
                }
                if prev_level - level > 3 {
                    return false
                }
            }
        }
        prev_level = level;
    }
    return true
}