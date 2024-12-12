use std::collections::HashMap;
use std::fs;

pub struct Day11{
    cache: HashMap<i64, Vec<i64>>
}

impl Day11{
    pub fn new() -> Day11{
        Day11{
            cache: HashMap::new()
        }
    }

    pub fn part1(&mut self) -> i64{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-11/src/day_11_input")
            .expect("Failed to read file");
        let mut line = String::new();
        self.cache = HashMap::new();

        for l in input.lines(){
            line = l.to_string();
        }

        let mut values: Vec<i64> = Vec::new();
        for char in line.split(" "){
            values.push(char.parse::<i64>().unwrap());
        }

        for _ in 0..5{
            let current_values = values.clone();
            values = Vec::new();
            for value in current_values{
                let cached_value = self.cache.get(&value);
                if cached_value.is_some(){
                    let mut new_values = cached_value.unwrap().clone();
                    values.append(&mut new_values);
                }else{
                    let mut new_values = self.solve(value, 5);
                    self.cache.insert(value, new_values.clone());
                    values.append(&mut new_values);
                }
            }
        }

        return values.len() as i64
    }

    pub fn part2(&mut self) -> i64{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-11/src/day_11_input")
            .expect("Failed to read file");
        let mut line = String::new();
        self.cache = HashMap::new();

        for l in input.lines(){
            line = l.to_string();
        }

        let mut values: Vec<i64> = Vec::new();
        for char in line.split(" "){
            values.push(char.parse::<i64>().unwrap());
        }

        for i in 0..2{
            println!("{}: {:?}", i, values.len());
            let current_values = values.clone();
            values = Vec::new();
            for value in current_values{
                let cached_value = self.cache.get(&value);
                if cached_value.is_some(){
                    let mut new_values = cached_value.unwrap().clone();
                    values.append(&mut new_values);
                }else{
                    let mut new_values = self.solve(value, 25);
                    self.cache.insert(value, new_values.clone());
                    values.append(&mut new_values);
                }
            }
        }

        let mut sum = 0;
        for value in values{
            let cached_value = self.cache.get(&value);
            if cached_value.is_some() {
                sum += cached_value.unwrap().len() as i64;
            }else{
                let new_values = self.solve(value, 25);
                self.cache.insert(value, new_values.clone());
                sum += new_values.len() as i64;
            }
        }

        return sum;
    }

    fn solve(&self, start:i64, count:i32) -> Vec<i64>{
        let mut values: Vec<i64> = Vec::new();
        values.push(start);
        let mut cache: HashMap<i64, (i64, i64)> = HashMap::new();
        
        for _ in 0..count {
            let current_line = values.clone();
            values = Vec::new();
            for value in current_line{
                if value == 0{
                    values.push(1);
                }else if value.to_string().len() % 2 == 0{
                    let cached_value = cache.get(&value);
                    if cached_value.is_some(){
                        let (val_1_int, val_2_int) = cached_value.unwrap();
                        values.push(*val_1_int);
                        values.push(*val_2_int);
                    }else{
                        let mut val_1 = String::new();
                        let mut val_2 = String::new();
                        let length = value.to_string().len();
                        for (i, char) in value.to_string().chars().enumerate(){
                            if i < length/2 {
                                val_1 += char.to_string().as_str();
                            } else{
                                val_2 += char.to_string().as_str()
                            }
                        }
                        let val_1_int = val_1.parse::<i64>().unwrap();
                        let val_2_int = val_2.parse::<i64>().unwrap();
                        cache.insert(value, (val_1_int, val_2_int));
                        values.push(val_1_int);
                        values.push(val_2_int);
                    }
                }else{
                    values.push(value*2024);
                }
            }
        }
        return values;

    }

    
}