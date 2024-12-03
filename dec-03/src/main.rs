use aoc_utils::get_lines;
use regex::Regex;

static DEBUG: bool = false;

fn main() {
    let data = build_input(&"input.txt".to_owned());
    println!("part1: {}", part1(data.clone()));
    println!("part2: {}", part2(data));
}

fn build_input(file: &String) -> String {
    let lines = get_lines(file);
    let mut data = String::new();
    lines.into_iter().for_each(|l| 
        data.push_str(&l.unwrap().to_owned())
    );

    data
}

fn part1(input: String) -> i64 {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut final_nums = vec![];
    for (_, [first, second]) in regex.captures_iter(input.as_str()).map(|r| r.extract()) {                     
        let first = first.to_string().parse::<i64>().unwrap();
        let second = second.to_string().parse::<i64>().unwrap();
        if DEBUG { println!("({},{})", first, second); }
        
        final_nums.push(first*second);
    };

    final_nums.iter().sum()
}

fn part2(input: String) -> i64 {
    let regex = Regex::new(r"(mul\([0-9]+,[0-9]+\)|don\'t\(\)|do\(\))").unwrap();
    let mut new_muls = vec![];
    let mut skip = false;
    for (_, [op]) in regex.captures_iter(input.as_str()).map(|r| r.extract()) {        
        if DEBUG { println!("op: {}", op); }

        if op.starts_with("don't") { skip = true; }
        else if op.starts_with("do") { skip = false; }
        else if !skip { new_muls.push(op); }
    };

    part1(new_muls.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = build_input(&"input-sample.txt".to_owned());
        assert_eq!(161, part1(data));
    }

    #[test]
    fn test_part2() {
        let data = build_input(&"input-sample2.txt".to_owned());
        assert_eq!(48, part2(data));
    }
}
