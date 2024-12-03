use aoc_utils::get_lines;

static DEBUG: bool = false;

fn main() {
    let input = build_input(&"input.txt".to_owned());
    println!("part1: {}", part1(input.clone()));
    println!("part2: {}", part2(input));
}

fn build_input(file: &String) -> Vec<Vec<i32>> {
    let lines = get_lines(file);
    let data: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|l| 
            l.unwrap()
            .split_whitespace()
            .map(|t| 
                t.parse::<i32>()
                .expect("NaN"))
            .collect::<Vec<i32>>())
        .collect();

    data
}

fn is_safe(v: Vec<i32>) -> bool {
    let is_safe_decreasing = v.is_sorted_by(|a,b| (a > b) && ((a-b).abs() <= 3));
    let is_safe_increasing = v.is_sorted_by(|a,b| (a < b) && ((a-b).abs() <= 3));
    if DEBUG {
        println!("v: {:?}", v);
        println!("Increasing?: {}", is_safe_increasing);
        println!("Decreasing?: {}", is_safe_decreasing);
        println!();
    }

    is_safe_decreasing || is_safe_increasing
}

fn part1(input: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0_i32;
    if DEBUG { println!("Part1 debug: "); }

    input.iter()
        .for_each(|v| 
            if is_safe(v.clone()) { 
                safe_reports+=1 
            }
        );

    safe_reports
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0_i32;
    if DEBUG { println!("Part2 debug: "); }

    for v in input {
        for i in 0..v.len() {
            let mut t = v.clone();
            t.remove(i);
            if is_safe(t) {
                safe_reports += 1;
                break;
            }
        }
    }

    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = build_input(&"input-sample.txt".to_owned());
        assert_eq!(2, part1(data));
    }

    #[test]
    fn test_part2() {
        let data = build_input(&"input-sample.txt".to_owned());
        assert_eq!(4, part2(data));
    }
}