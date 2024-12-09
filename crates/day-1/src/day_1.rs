use std::fs;

pub struct Day1{
}

impl Day1{
    pub fn new() -> Day1{
        Day1{}
    }

    pub fn part1(&mut self) -> i32{
        let contents = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-1/src/day_1_input")
        .expect("Failed to read file");

        let mut list_one: Vec<i32> = Vec::new();
        let mut list_two: Vec<i32> = Vec::new();

        for line in contents.lines() {
            let values: Vec<&str> = line.split("   ").collect();
            let int_one = values[0].parse::<i32>().unwrap();
            let int_two = values[1].parse::<i32>().unwrap();
            list_one.push(int_one); 
            list_two.push(int_two); 
        }


        list_one.sort();
        list_two.sort();
        let mut result = 0;

        for vals in list_one.iter().zip(list_two.iter()){
            let (val1, val2) = vals;
            let diff = if val1 > val2 {
                val1-val2
            } else {
                val2-val1
            };
            result += diff;
        }

        return result;
    }

    pub fn part2(&mut self) -> i32{
        let contents = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-1/src/day_1_input")
        .expect("Failed to read file");

        let mut list_one: Vec<i32> = Vec::new();
        let mut list_two: Vec<i32> = Vec::new();

        for line in contents.lines() {
            let values: Vec<&str> = line.split("   ").collect();
            let int_one = values[0].parse::<i32>().unwrap();
            let int_two = values[1].parse::<i32>().unwrap();
            list_one.push(int_one); 
            list_two.push(int_two); 
        }


        let mut result = 0;

        for val in list_one.iter(){
            let count = list_two.iter().filter(|&x| x == val).count() as i32;
            result += val * count;
        }

        return result;
    }
}