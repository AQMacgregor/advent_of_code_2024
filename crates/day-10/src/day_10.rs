use std::{collections::HashSet, fs};

pub struct Day10{
    grid: Vec<Vec<i32>>
}

impl Day10{
    pub fn new() -> Day10{
        Day10{
            grid: Vec::new()
        }
    }

    pub fn part1(&mut self) -> i32{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-10/src/day_10_input")
            .expect("Failed to read file");

        let mut starts = Vec::new();
        for (y,  line) in input.lines().enumerate(){
            let mut row = Vec::new();
            for (x, char) in line.chars().enumerate(){
                if char == '0' {
                    starts.push((x, y));
                }
                row.push(char.to_string().parse::<i32>().unwrap());
            }
            self.grid.push(row);
        }

        let mut total = 0;
        for start in starts{
            let mut nines = HashSet::new();
            for direction in self.search(start){
                let next_height = self.grid[direction.1][direction.0];
                if next_height == 1 {
                    for next_location in self.travel(direction){
                        nines.insert(next_location);
                    }
                }
            }
            let score = nines.len() as i32;
            total += score;
        }

        return total;
    }

    fn travel(&self, start:(usize, usize)) -> Vec<(usize,usize)>{
        let mut score = Vec::new();
        let current_height = self.grid[start.1][start.0];
        if current_height == 9 {
            score.push(start);
        }else{
            for location in self.search(start){
                let next_height = self.grid[location.1][location.0];
                if  next_height == current_height+1{
                    for next_location in self.travel(location){
                        score.push(next_location);
                    }
                }
            }
        }
        return score;
    }

    fn search(&self, (x, y):(usize, usize)) -> Vec<(usize, usize)> {
        let possible_directions = vec![
            self.go_north(x, y, 0, 1),
            self.go_south(x, y, 0, 1),
            self.go_east(x, y, 1, 0),
            self.go_west(x, y, 1, 0)
        ];

        return possible_directions.iter().filter(|f| (f.is_some())).map(|f| {f.unwrap()}).collect()
    }

    pub fn part2(&mut self) -> i32{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-10/src/day_10_input")
            .expect("Failed to read file");

        let mut starts = Vec::new();
        for (y,  line) in input.lines().enumerate(){
            let mut row = Vec::new();
            for (x, char) in line.chars().enumerate(){
                if char == '0' {
                    starts.push((x, y));
                }
                row.push(char.to_string().parse::<i32>().unwrap());
            }
            self.grid.push(row);
        }

        let mut total = 0;
        for start in starts{
            let mut nines = Vec::new();
            for direction in self.search(start){
                let next_height = self.grid[direction.1][direction.0];
                if next_height == 1 {
                    for next_location in self.travel(direction){
                        nines.push(next_location);
                    }
                }
            }
            let score = nines.len() as i32;
            total += score;
        }

        return total;
    }


    fn go_north(&self, x:usize, y:usize, _:usize, y_dis:usize) -> Option<(usize, usize)>{
        let new_x = x;
        let new_y = y + y_dis;
        if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
            return Some((new_x, new_y))
        }
        return None
    }
    fn go_south(&self, x:usize, y:usize, _:usize, y_dis:usize) -> Option<(usize, usize)>{
        if y >= y_dis{
            let new_x = x;
            let new_y = y - y_dis;
            if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
                return Some((new_x, new_y))
            }
        }
        return None
    }
    fn go_east(&self, x:usize, y:usize, x_dis:usize, _:usize) -> Option<(usize, usize)>{
        let new_x = x + x_dis;
        let new_y = y;
        if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
            return Some((new_x, new_y))
        }
        return None
    }
    fn go_west(&self, x:usize, y:usize, x_dis:usize, _:usize) -> Option<(usize, usize)>{
        if x >= x_dis{
            let new_x = x - x_dis;
            let new_y = y;
            if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
                return Some((new_x, new_y))
            }
        }
        return None
    }
}