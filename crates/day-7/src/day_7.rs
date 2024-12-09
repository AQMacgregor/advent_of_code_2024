use std::fs;

pub struct Day7{

}

struct Problem {
    total:i64, 
    values:Vec<i64>
}

impl Problem{
    pub fn new(total:i64, values:Vec<i64>) -> Problem{
        Problem{
            total,
            values
        }
    }

    pub fn get_total(&self) -> i64{
        return self.total
    }

    pub fn solve(&self, branch: fn(i64,i64)-> Vec<i64>) -> bool{
        let mut results = Vec::new();
        results.push(self.values[0]);

        for (i, result) in self.values.clone().iter().enumerate(){
            if i == 0 {
                continue;
            }

            let mut next_results = Vec::new();
            for value in results{
                let branchs = branch(value, *result);
                for branch in branchs{
                    next_results.push(branch);
                }
            }
            results = next_results.clone();
        }
        return results.contains(&self.total)
    }    
}

fn branch_1(value: i64, next:i64) -> Vec<i64>{
    return vec![value + next, value * next]
}

fn branch_2(value: i64, next:i64) -> Vec<i64>{
    let third = String::from(format!("{}{}", value.to_string(), next.to_string())).parse::<i64>().unwrap();
    return vec![value + next, value * next, third]
}

impl Day7{
    pub fn new() -> Day7{
        Day7{}
    }

    pub fn part1(&mut self) -> i64{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-7/src/day_7_input")
        .expect("Failed to read file");

        let mut problems = Vec::new();
        for line in input.lines(){
            let parts:Vec<&str> = line.split(": ").collect();
            let total = parts[0].parse::<i64>().unwrap();
            let values:Vec<i64> = parts[1].split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
            problems.push(Problem::new(total,values))
        }
        let mut count = 0;
        for problem in problems{
            if problem.solve(branch_1){
                count += problem.get_total();
            }
        }

        return count;
    }

    pub fn part2(&mut self) -> i64{
        let input = fs::read_to_string("/Users/macgrea/code/advent_of_code/crates/day-7/src/day_7_input")
        .expect("Failed to read file");

        let mut problems = Vec::new();
        for line in input.lines(){
            let parts:Vec<&str> = line.split(": ").collect();
            let total = parts[0].parse::<i64>().unwrap();
            let values:Vec<i64> = parts[1].split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
            problems.push(Problem::new(total,values))
        }
        let mut count = 0;
        for problem in problems{
            if problem.solve(branch_2){
                count += problem.get_total();
            }
        }

        return count;
    }
}