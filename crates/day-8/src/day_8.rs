use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

pub struct Day8{
    grid: Vec<Vec<char>>
}

impl Day8{
    pub fn new() -> Day8{
        Day8{
            grid:Vec::new()
        }
    }

    pub fn part1(&mut self) -> i64{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-8/src/day_8_input")
        .expect("Failed to read file");

        let mut antennas = HashMap::new();
        self.grid = Vec::new();
        for (y, line) in input.lines().enumerate(){
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate(){
                row.push(c);
                if c != '.' {
                    if !antennas.contains_key(&c){
                        antennas.insert(c, Vec::new());
                    }
                    let locations = antennas.get_mut(&c).unwrap();
                    locations.push((x, y));
                } 
            }
            self.grid.push(row);
        }

        let mut count = 0;
        for (_, locations) in antennas{
            for pair in locations.iter().permutations(2){
                let x_1 = pair[0].0;
                let y_1 = pair[0].1;
                let x_2 = pair[1].0;
                let y_2 = pair[1].1;

                let x_dis = x_1.abs_diff(x_2); 
                let y_dis = y_1.abs_diff(y_2); 

                let mut x_3: Option<usize> = None;
                let mut y_3: Option<usize> = None;
                if x_1 == x_2 && y_1< y_2{ // North
                    x_3 = Some(x_2);
                    y_3 = Some(y_2 + y_dis);
                }else if x_1 == x_2 && y_1> y_2{ // South
                    if y_2 >= y_dis {
                        x_3 = Some(x_2);
                        y_3 = Some(y_2 - y_dis);
                    }
                }else if y_1 == y_2 && x_1< x_2{ // East
                    x_3 = Some(x_2 + x_dis);
                    y_3 = Some(y_2);
                }else if y_1 == y_2 && x_1> x_2{ // West
                    if x_2 >= x_dis{
                        x_3 = Some(x_2 - x_dis);
                        y_3 = Some(y_2);
                    }
                } else if x_1 < x_2 && y_1 < y_2{ // North East
                    x_3 = Some(x_2 + x_dis);
                    y_3 = Some(y_2 + y_dis);
                } else if x_1 < x_2 && y_1 > y_2{ // South East
                    if y_2 >= y_dis {
                        x_3 = Some(x_2 + x_dis);
                        y_3 = Some(y_2 - y_dis);
                    }
                } else if x_1 > x_2 && y_1 < y_2{ // North West
                    if x_2 >= x_dis {
                        x_3 = Some(x_2 - x_dis);
                        y_3 = Some(y_2 + y_dis);
                    }
                } else if x_1 > x_2 && y_1 > y_2{ // South West
                    if x_2 >= x_dis && y_2 >= y_dis {
                        x_3 = Some(x_2 - x_dis);
                        y_3 = Some(y_2 - y_dis);
                    }
                } else {
                    println!("{},{} {},{}", x_1, y_1, x_2,y_2)
                }
                
                if x_3.is_some() && y_3.is_some(){
                    let x = x_3.unwrap();
                    let y = y_3.unwrap();
                    if y < self.grid.len() && x < self.grid[y].len() {    
                        if self.grid[y][x] != '#'{
                            count+=1;
                            self.grid[y][x] = '#'
                        }
                    }
                }
            }
        }
        return count;
    }

    pub fn part2(&mut self) -> i64{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-8/src/day_8_input")
        .expect("Failed to read file");

        let mut antennas = HashMap::new();
        self.grid = Vec::new();
        for (y, line) in input.lines().enumerate(){
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate(){
                row.push(c);
                if c != '.' {
                    if !antennas.contains_key(&c){
                        antennas.insert(c, Vec::new());
                    }
                    let locations = antennas.get_mut(&c).unwrap();
                    locations.push((x, y));
                } 
            }
            self.grid.push(row);
        }

        let mut count = 0;
        for (_, locations) in antennas{
            for pair in locations.iter().permutations(2){
                let x_1 = pair[0].0;
                let y_1 = pair[0].1;
                let x_2 = pair[1].0;
                let y_2 = pair[1].1;

                let x_dis = x_1.abs_diff(x_2); 
                let y_dis = y_1.abs_diff(y_2); 

                let mut xys: Vec<(usize, usize)> = Vec::new();
                xys.push((x_2, y_2));
                if x_1 == x_2 && y_1< y_2{ // North
                    let mut xy = self.go_north(x_2, y_2, x_dis, y_dis);
                    while xy.is_some(){
                        xys.push(xy.unwrap());
                        let (x, y ) = xy.unwrap();
                        xy = self.go_north(x, y, x_dis, y_dis);
                    }
                    
                }else if x_1 == x_2 && y_1> y_2{ // South
                    let mut xy = self.go_south(x_2, y_2, x_dis, y_dis);
                    while xy.is_some(){
                        xys.push(xy.unwrap());
                        let (x, y ) = xy.unwrap();
                        xy = self.go_south(x, y, x_dis, y_dis);
                    }
                }else if y_1 == y_2 && x_1< x_2{ // East
                    let mut xy = self.go_east(x_2, y_2, x_dis, y_dis);
                    while xy.is_some(){
                        xys.push(xy.unwrap());
                        let (x, y ) = xy.unwrap();
                        xy = self.go_east(x, y, x_dis, y_dis);
                    }
                }else if y_1 == y_2 && x_1> x_2{ // West
                    let mut xy = self.go_west(x_2, y_2, x_dis, y_dis);
                    while xy.is_some(){
                        xys.push(xy.unwrap());
                        let (x, y ) = xy.unwrap();
                        xy = self.go_west(x, y, x_dis, y_dis);
                    }
                } else if x_1 < x_2 && y_1 < y_2{ // North East
                    let mut xy = self.go_north_east(x_2, y_2, x_dis, y_dis);
                    while xy.is_some(){
                        xys.push(xy.unwrap());
                        let (x, y ) = xy.unwrap();
                        xy = self.go_north_east(x, y, x_dis, y_dis);
                    }
                } else if x_1 < x_2 && y_1 > y_2{ // South East
                    let mut xy = self.go_south_east(x_2, y_2, x_dis, y_dis);
                    while xy.is_some(){
                        xys.push(xy.unwrap());
                        let (x, y ) = xy.unwrap();
                        xy = self.go_south_east(x, y, x_dis, y_dis);
                    }
                } else if x_1 > x_2 && y_1 < y_2{ // North West
                    let mut xy = self.go_north_west(x_2, y_2, x_dis, y_dis);
                    while xy.is_some(){
                        xys.push(xy.unwrap());
                        let (x, y ) = xy.unwrap();
                        xy = self.go_north_west(x, y, x_dis, y_dis);
                    }
                } else if x_1 > x_2 && y_1 > y_2{ // South West
                    let mut xy = self.go_south_west(x_2, y_2, x_dis, y_dis);
                    while xy.is_some(){
                        xys.push(xy.unwrap());
                        let (x, y ) = xy.unwrap();
                        xy = self.go_south_west(x, y, x_dis, y_dis);
                    }
                } else {
                    println!("{},{} {},{}", x_1, y_1, x_2,y_2)
                }
                for xy in xys{
                    let (x, y) = xy;
                    if y < self.grid.len() && x < self.grid[y].len() {    
                        if self.grid[y][x] != '#'{
                            count+=1;
                            self.grid[y][x] = '#'
                        }
                    }
                }
            }
        }
        return count;
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
    fn go_north_east(&self, x:usize, y:usize, x_dis:usize, y_dis:usize) -> Option<(usize, usize)>{
        let new_x = x+x_dis;
        let new_y = y + y_dis;
        if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
            return Some((new_x, new_y))
        }
        return None
    }
    fn go_south_east(&self, x:usize, y:usize, x_dis:usize, y_dis:usize) -> Option<(usize, usize)>{
        if y >= y_dis {
            let new_x = x+x_dis;
            let new_y = y - y_dis;
            if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
                return Some((new_x, new_y))
            }
        }
        return None
    }
    fn go_north_west(&self, x:usize, y:usize, x_dis:usize, y_dis:usize) -> Option<(usize, usize)>{
        if x >= x_dis {
            let new_x = x-x_dis;
            let new_y = y + y_dis;
            if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
                return Some((new_x, new_y))
            }
        }
        return None
    }
    fn go_south_west(&self, x:usize, y:usize, x_dis:usize, y_dis:usize) -> Option<(usize, usize)>{
        if x >= x_dis && y >= y_dis {
            let new_x = x-x_dis;
            let new_y = y - y_dis;
            if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
                return Some((new_x, new_y))
            }
        }
        return None
    }
}