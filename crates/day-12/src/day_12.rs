use std::fs;

pub struct Day12{
    grid: Vec<Vec<char>>
}

impl Day12{
    pub fn new() -> Day12{
        Day12{
            grid: Vec::new()
        }
    }

    pub fn part1(&mut self) -> i32{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-12/src/day_12_input")
            .expect("Failed to read file");

        for line in input.lines(){
            let mut row = Vec::new();
            for char in line.chars(){
                row.push(char);
            }
            self.grid.push(row);
        }

        let mut perimeters = Vec::new();
        for (j, row) in self.grid.iter().enumerate(){
            let mut row_perimeters = Vec::new();
            for (i, char) in row.iter().enumerate(){
                let mut perimeter: i32 = 4;
                if let Some(new_location) = self.go_north(i, j){
                    if self.grid[new_location.1][new_location.0] == *char{
                        perimeter -= 1;
                    }
                }
                if let Some(new_location) = self.go_south(i, j){
                    if self.grid[new_location.1][new_location.0] == *char{
                        perimeter -= 1;
                    }
                }
                if let Some(new_location) = self.go_east(i, j){
                    if self.grid[new_location.1][new_location.0] == *char{
                        perimeter -= 1;
                    }
                }
                if let Some(new_location) = self.go_west(i, j){
                    if self.grid[new_location.1][new_location.0] == *char{
                        perimeter -= 1;
                    }
                }
                row_perimeters.push(perimeter);
            }
            perimeters.push(row_perimeters);
        }

        let mut counted = Vec::new();

        let mut total = 0;
        for (j, row) in self.grid.iter().enumerate(){
            for (i, char) in row.iter().enumerate(){
                if counted.contains(&(i, j)){
                    continue;
                }
                let mut visited = Vec::new();
                self.search(i, j, char, &mut visited);
                let mut perimeter = 0;
                let mut size = 0;
                for value in visited { 
                    size += 1;
                    counted.push(value);
                    perimeter += perimeters[value.1][value.0];
                } 
                total += perimeter * size;
            }
        }

        total
    }

    fn is_corner(&self, char: char, diagonal:Option<(usize,usize)>, direction_1: Option<(usize,usize)>, direction_2:Option<(usize,usize)>) -> bool{
        if diagonal.is_some_and(|l| {self.grid[l.1][l.0] != char}) || diagonal.is_none() {
            if (direction_1.is_some_and(|l| {self.grid[l.1][l.0] != char}) || direction_1.is_none()) 
                && (direction_2.is_some_and(|l| {self.grid[l.1][l.0] != char}) || direction_2.is_none()){
                true
            }else if direction_1.is_some_and(|l| {self.grid[l.1][l.0] == char}) 
                && direction_2.is_some_and(|l| {self.grid[l.1][l.0] == char}) {
                true
            }else{
                false
            }
        }else{
            if direction_1.is_some_and(|l| {self.grid[l.1][l.0] != char}) &&
            direction_2.is_some_and(|l| {self.grid[l.1][l.0] != char}){
                    true
                }else{
                    false
                }
        }
    }

    pub fn part2(&mut self) -> i32{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-12/src/day_12_input")
            .expect("Failed to read file");

        for line in input.lines(){
            let mut row = Vec::new();
            for char in line.chars(){
                row.push(char);
            }
            self.grid.push(row);
        }

        let mut corners = Vec::new();
        for (j, row) in self.grid.iter().enumerate(){
            let mut row_corners = Vec::new();
            for (i, char) in row.iter().enumerate(){
                let char = *char;
                let mut corner: i32 = 0;
                let north = self.go_north(i, j);
                let east = self.go_east(i, j); 
                let west = self.go_west(i, j); 
                let south = self.go_south(i, j);

                let north_east = self.go_north_east(i, j);
                if self.is_corner(char, north_east, north, east){
                    corner += 1;
                }
                let north_west = self.go_north_west(i, j);
                if self.is_corner(char, north_west, north, west){
                    corner += 1;
                }
                let south_east = self.go_south_east(i, j);
                if self.is_corner(char, south_east, south, east){
                    corner += 1;
                }
                let south_west = self.go_south_west(i, j);
                if self.is_corner(char, south_west, south, west){
                    corner += 1;
                }

                row_corners.push(corner);
            }
            corners.push(row_corners);
        }

        let mut counted = Vec::new();

        let mut total = 0;
        for (j, row) in self.grid.iter().enumerate(){
            for (i, char) in row.iter().enumerate(){
                if counted.contains(&(i, j)){
                    continue;
                }
                let mut visited = Vec::new();
                self.search(i, j, char, &mut visited);
                let mut perimeter = 0;
                let mut size = 0;
                for value in visited { 
                    size += 1;
                    counted.push(value);
                    perimeter += corners[value.1][value.0];
                } 
                total += perimeter * size;
            }
        }

        total
    }

    fn search(&self, x:usize, y:usize, char:&char, visited:&mut Vec<(usize,usize)>){
        visited.push((x,y));
        if let Some(new_location) = self.go_north(x, y){
            if self.grid[new_location.1][new_location.0] == *char && !visited.contains(&new_location){
                self.search(new_location.0, new_location.1, char, visited)
            }
        }
        if let Some(new_location) = self.go_south(x, y){
            if self.grid[new_location.1][new_location.0] == *char && !visited.contains(&new_location){
                self.search(new_location.0, new_location.1, char,  visited)
            }
        }
        if let Some(new_location) = self.go_east(x, y){
            if self.grid[new_location.1][new_location.0] == *char && !visited.contains(&new_location){
                self.search(new_location.0, new_location.1, char, visited)
            }
        }
        if let Some(new_location) = self.go_west(x, y){
            if self.grid[new_location.1][new_location.0] == *char && !visited.contains(&new_location){
                self.search(new_location.0, new_location.1, char, visited)
            }
        }
    }

    fn go_north(&self, x:usize, y:usize) -> Option<(usize, usize)>{
        let new_x = x;
        let new_y = y + 1;
        if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
            return Some((new_x, new_y))
        }
        None
    }

    fn go_north_east(&self, x:usize, y:usize) -> Option<(usize, usize)>{
        let new_x = x + 1;
        let new_y = y + 1;
        if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
            return Some((new_x, new_y))
        }
        None
    }

    fn go_north_west(&self, x:usize, y:usize) -> Option<(usize, usize)>{
        if x >= 1{
            let new_x = x - 1;
            let new_y = y + 1;
            if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
                return Some((new_x, new_y))
            }
        }
        None
    }

    fn go_south(&self, x:usize, y:usize) -> Option<(usize, usize)>{
        if y >= 1{
            let new_x = x;
            let new_y = y - 1;
            if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
                return Some((new_x, new_y))
            }
        }
        None
    }
    fn go_south_west(&self, x:usize, y:usize) -> Option<(usize, usize)>{
        if y >= 1 && x >= 1{
            let new_x = x - 1;
            let new_y = y - 1;
            if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
                return Some((new_x, new_y))
            }
        }
        None
    }
    fn go_south_east(&self, x:usize, y:usize) -> Option<(usize, usize)>{
        if y >= 1{
            let new_x = x + 1;
            let new_y = y - 1;
            if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
                return Some((new_x, new_y))
            }
        }
        None
    }

    fn go_east(&self, x:usize, y:usize) -> Option<(usize, usize)>{
        let new_x = x + 1;
        let new_y = y;
        if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
            return Some((new_x, new_y))
        }
        None
    }

    fn go_west(&self, x:usize, y:usize) -> Option<(usize, usize)>{
        if x >= 1{
            let new_x = x - 1;
            let new_y = y;
            if new_x < self.grid.len() && new_y < self.grid[new_x].len() {
                return Some((new_x, new_y))
            }
        }
        None
    }
}