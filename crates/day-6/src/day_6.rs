use std::{collections::HashMap, fs};

pub struct Day6{
    grid :Vec<Vec<char>>
}

pub struct Guard{
    position: (usize, usize),
    direction: char
}
type Position = (usize, usize);

impl Guard {
    pub fn new() -> Guard{
        Guard { position: (0, 0), direction: ('N') }
    }

    pub fn position(&mut self, positon: (usize, usize), direction: char){
        self.direction = direction;
        self.position = positon;
    }

    pub fn current_postion(&self) -> Position{
        return self.position;
    }

    pub fn current_direction(&self) -> char{
        return self.direction;
    }

    pub fn facing(&mut self) -> Position{
        match self.direction {
            'N' => (self.position.0, self.position.1 - 1),
            'E' => (self.position.0 + 1, self.position.1),
            'S' => (self.position.0, self.position.1 + 1),
            'W' => (self.position.0 - 1, self.position.1),
            _ => self.position
        }
    }

    pub fn move_forward(&mut self){
        match self.direction {
            'N' => self.position((self.position.0, self.position.1 - 1), 'N'),
            'E' => self.position((self.position.0 + 1, self.position.1), 'E'),
            'S' => self.position((self.position.0, self.position.1 + 1), 'S'),
            'W' => self.position((self.position.0 - 1, self.position.1), 'W'),
            _ => {}
        }
    }

    pub fn turn_right(&mut self){
        match self.direction {
            'N' => self.position((self.position.0, self.position.1), 'E'),
            'E' => self.position((self.position.0, self.position.1),'S'),
            'S' => self.position((self.position.0, self.position.1), 'W'),
            'W' => self.position((self.position.0, self.position.1), 'N'),
            _ => {}
        }
    }
}


impl Day6{
    pub fn new() -> Day6{
        Day6{
            grid: Vec::new()
        }
    }

    fn build_grid(&mut self) -> Guard{
        let grid = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-6/src/day_6_input")
        .expect("Failed to read file");
        self.grid = Vec::new();

        let mut guard: Guard = Guard::new();

        for (y, line) in grid.lines().enumerate(){
            let mut grid_line: Vec<char> = Vec::new();
            for (x, c) in line.chars().enumerate(){
                match c {
                    '^' =>  guard.position((x, y), 'N'),
                    '>' =>  guard.position((x, y), 'E'),
                    'V' =>  guard.position((x, y), 'S'),
                    '<' =>  guard.position((x, y), 'W'),
                    _ => {}
                }
                grid_line.push(c);
            }
            self.grid.push(grid_line);
        }
        return guard;
    }

    

    pub fn part1(&mut self) -> i32{
        let mut guard = self.build_grid();
        
        let mut count = 0;
        while !self.is_at_edge(&guard){
            let guard_position = guard.current_postion();
            let guard_facing = guard.facing();
            if self.grid[guard_facing.1][guard_facing.0] == '#' {
                guard.turn_right();
            }else{
                guard.move_forward();
            }
            if self.grid[guard_position.1][guard_position.0] != 'X' {
                count += 1;
                self.grid[guard_position.1][guard_position.0] = 'X'
            }
        }
        
        // Last stop before leaving
        count += 1;
        return count;
    }

    pub fn part2(&mut self) -> i32{
        self.build_grid();
        let y_max = self.grid.len();
        let x_max = self.grid[y_max-1].len();
        let mut count:i32 = 0;

        for y in 0..y_max {
            for x in 0..x_max {
                let mut guard = self.build_grid();
                self.grid[y][x] = '#';
                let mut positions = HashMap::new();
                let mut is_loop = false;
                while !self.is_at_edge(&guard){
                    let guard_position = guard.current_postion();
                    let guard_direction = guard.current_direction();
                    let guard_facing = guard.facing();
                    if self.grid[guard_facing.1][guard_facing.0] == '#' {
                        guard.turn_right();
                    }else{
                        guard.move_forward();
                    }
                    let key = String::from(format!("{}-{}", guard_position.0, guard_position.1));
                    if self.grid[guard_position.1][guard_position.0] != 'X' {
                        positions.insert(key, guard_direction);
                        self.grid[guard_position.1][guard_position.0] = 'X'
                    } else if positions.get(&key) == Some(&guard_direction) {
                        is_loop = true;
                        break
                    }
                }
        
                if is_loop {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn is_at_edge(&self, guard: &Guard) -> bool{
        let y_max = self.grid.len();
        let x_max = self.grid[y_max-1].len();

        if guard.current_postion().0 == 0 && guard.current_direction() == 'W'{
            return true;
        }
        if guard.current_postion().0 == x_max-1 && guard.current_direction() == 'E'{
            return true;
        }
        if guard.current_postion().1 == 0 && guard.current_direction() == 'N'{
            return true;
        }
        if guard.current_postion().1 == y_max-1 && guard.current_direction() == 'S' {
            return true;
        }
        return false
    }
}