use std::fs;

pub struct Day9{
}

impl Day9{
    pub fn new() -> Day9{
        Day9{}
    }

    pub fn part1(&mut self) -> i64{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-9/src/day_9_input")
            .expect("Failed to read file");

        let mut file = Vec::new();
        for line in input.lines(){
            file = line.chars().map(|c| {c.to_string().parse::<i32>().unwrap()}).collect();
        }

        let mut processed_file = Vec::new();
        let mut id = 0;
        for (index, value) in file.iter().enumerate(){
            for _ in 0..*value{
                if index % 2 == 0{
                    processed_file.push(id.to_string());
                }else{
                    processed_file.push(".".to_string());
                }
            }
            if index % 2 == 0{
                id += 1
            }
        }   

        let mut done = false;
        let mut ordered_file = processed_file.clone();
        while !done{
            let mut backward_file = ordered_file.clone();
            backward_file.reverse();
            let mut replace_i = 0;
            let mut replace_j = 0;
            for (i, value) in ordered_file.iter().enumerate(){
                if value == "."{
                    replace_i = i;
                    for (j, replace) in backward_file.iter().enumerate(){
                        let j_index = ordered_file.len() - 1 - j;
                        if replace != "."{
                            replace_j = j_index;
                            break;
                        }else if i == j_index{
                            done = true;
                            break;
                        }
                    }
                    break;
                }
            }
            if !done{
                println!("{}<->{}", replace_i, replace_j);
                ordered_file.swap(replace_i, replace_j);
            }
        }

        let mut checksome:i64 = 0;
        for (i, value) in ordered_file.iter().enumerate(){
            if value == "."{
                break;
            }
            checksome += i as i64 * value.parse::<i64>().unwrap();
        }
        
        return checksome;
    }

    pub fn part2(&mut self) -> i64{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-9/src/day_9_input")
            .expect("Failed to read file");

        let mut file = Vec::new();
        for line in input.lines(){
            file = line.chars().map(|c| {c.to_string().parse::<i32>().unwrap()}).collect();
        }

        let mut processed_file = Vec::new();
        let mut id = 0;
        for (index, value) in file.iter().enumerate(){
            if index % 2 == 0{
                processed_file.push((id.to_string(), *value));
            }else{
                processed_file.push((".".to_string(), *value));
            }
            if index % 2 == 0{
                id += 1
            }
        }   

        let mut ordered_file = processed_file.clone();
        println!("{:?}", ordered_file);
        let mut backward_file = ordered_file.clone();
        backward_file.reverse();

        let mut replace_i = 0;
        let mut replace_j = 0;
        let mut new_value = 0;
        let mut found_swap = false;

        for (_, replace) in backward_file.iter().enumerate(){
            if replace.0 != "."{
                for (i, value) in ordered_file.iter().enumerate(){
                    if replace.0 == value.0{
                        break;
                    }
                    if value.0 == "." && value.1 >= replace.1 {
                        new_value = value.1 - replace.1;
                        for (k , s) in ordered_file.iter().enumerate() { 
                            if s.0 == replace.0{
                                replace_j = k;
                            }
                        }
                        replace_i = i;
                        found_swap = true;
                        break;
                    }
                }
            }
        
            if found_swap{
                ordered_file.swap(replace_i, replace_j);
                ordered_file[replace_j].1 -= new_value;
                if new_value != 0{
                    ordered_file.insert(replace_i + 1, (".".to_string(), new_value));
                }
                let mut condenced_file = Vec::new();
                for value in ordered_file.iter(){
                    if value.0 != "."{
                        condenced_file.push(value.clone());
                    }else{
                        let last_index = condenced_file.len() -1;
                        if condenced_file[last_index].0 == "."{
                            condenced_file[last_index].1 += value.1;
                        }else{
                            condenced_file.push(value.clone());
                        }
                    }
                }
                ordered_file = condenced_file.clone();
                found_swap = false;
            }
        }
        println!("{:?}", ordered_file);

        let mut final_file = Vec::new();
        for (_, value) in ordered_file.iter().enumerate(){
            for _ in 0..value.1{
                final_file.push(value.0.clone());
            }
        }
        
        let mut checksome:i64 = 0;
        for (i, value) in final_file.iter().enumerate(){
            if value != "."{
                checksome += i as i64 * value.parse::<i64>().unwrap();
            }
        }
        
        return checksome;
    }
}