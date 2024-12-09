use std::fs;

pub struct Part2{
    wordsearch: Vec<Vec<char>>,
    x_max: i32,
    y_max: i32
}

impl Part2{
    pub fn new() -> Part2{
        Part2{
            wordsearch: Vec::new(),
            x_max: 0,
            y_max: 0
        }
    }

    pub fn solve(&mut self) -> i32{
        let contents = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-4/src/day_4_input")
            .expect("Failed to read file");
        
        for line in contents.lines(){
            let mut workline: Vec<char> = Vec::new();
            let mut x_row = 0;
            for c in line.chars(){
                workline.push(c);
                x_row+=1
            }
            self.wordsearch.push(workline);
            self.y_max+=1;
            self.x_max = x_row;
        }

        let mut count = 0;
        for y in 0..self.y_max{
            for x in 0..self.x_max{
                count += self.check_wordsearch( x as i32, y as i32)
            }
        }
        return count
    }
    
    fn check_wordsearch(&self, x:i32, y: i32) -> i32{
        let mut count = 0;
        if self.check_spot(x, y) == 'A'{
            if self.check_north(x, y) {count += 1}
            if self.check_south(x, y) {count += 1}
            if self.check_east(x, y) {count += 1}
            if self.check_west(x, y) {count += 1}
        }
        return count
    }

    fn check_west(&self, x:i32, y: i32) -> bool{
        if self.check_spot(x-1, y-1) == 'M' 
        && self.check_spot(x+1, y-1)  == 'S' 
        && self.check_spot(x-1, y+1)  == 'M' 
        && self.check_spot(x+1, y+1) == 'S' {
            return true
        }
        return false
    }

    fn check_south(&self, x:i32, y: i32) -> bool{
        if self.check_spot(x-1, y-1) == 'M' 
        && self.check_spot(x+1, y-1)  == 'M' 
        && self.check_spot(x-1, y+1)  == 'S' 
        && self.check_spot(x+1, y+1) == 'S' {
            return true
        }
        return false
    }

    fn check_east(&self, x:i32, y: i32) -> bool{
        if self.check_spot(x-1, y-1) == 'S' 
        && self.check_spot(x+1, y-1)  == 'M' 
        && self.check_spot(x-1, y+1)  == 'S' 
        && self.check_spot(x+1, y+1) == 'M' {
            return true
        }
        return false
    }

    fn check_north(&self, x:i32, y: i32) -> bool{
        if self.check_spot(x-1, y-1) == 'S' 
        && self.check_spot(x+1, y-1)  == 'S' 
        && self.check_spot(x-1, y+1)  == 'M' 
        && self.check_spot(x+1, y+1) == 'M' {
            return true
        }
        return false
    }

    fn check_spot(&self, x:i32, y: i32) -> char{
        if x > -1 && y > -1 && x  < self.x_max && y < self.y_max {
            return self.wordsearch[y as usize][x as usize]
        }else{
            return '#'
        }
    }

}
/*
[][][][0,3][][][]
[][][][][][][]
[][][][][][][]
[-3,0][-2,0][-1,0][0,0][1,0][2,0][3,0]
[][][][][][][]
[][][][][][][]
[-3,-3][][][0,-3][][][]
*/