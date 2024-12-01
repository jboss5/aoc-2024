use aoc_utils::get_lines;
use std::{collections::HashMap, fs::File, io::{BufReader, Lines}};

struct Data {
    left: Vec<i32>,
    right: Vec<i32>
}

static DEBUG: bool = false;

fn main() {
    let lines = get_lines(&"input.txt".to_owned());
    let data = part1(lines);
    let final_dist = calc_distances(&data);
    
    if DEBUG {
        for i in 0..data.left.len() {
            println!("{}  {}", data.left[i], data.right[i]);
        }
    }

    println!("Part1: {}", final_dist);
    println!("Part2: {}", part2(&data));
}

fn calc_distances(data: &Data) -> i64 {
    let mut final_dist = 0_i64;
    for i in 0..data.left.len() {
        // need abs as makes no sense for a negative distance
        final_dist += (data.right[i] - data.left[i]).abs() as i64;
    }

    final_dist
}

fn part1(lines: Lines<BufReader<File>>) -> Data {
    let mut data = Data { left: Vec::<i32>::new(), right: Vec::<i32>::new() };
    for l in lines {
        let t: Vec<i32> = l.unwrap().to_owned().split_whitespace().map(|s| s.parse::<i32>().expect("NaN")).collect();
        data.left.push(t[0]);
        data.right.push(t[1]);
    }

    data.left.sort();
    data.right.sort();

    data
}

fn part2(data: &Data) -> i64 {
    let mut occ_map: HashMap<i32, i64> = HashMap::new();
    for right_val in &data.right {
        let mut tmp = *occ_map.entry(*right_val).or_insert(0);
        tmp += 1;
        occ_map.insert(*right_val, tmp);
    }

    if DEBUG {
        for ele in &occ_map {
            println!("{} -> {}", ele.0, ele.1);
        }
    }

    let mut final_val = 0_i64;
    for left_val in &data.left {
        final_val += (*left_val as i64) * occ_map.get(&left_val).or(Some(&0_i64)).unwrap();
    }

    final_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sim_map() -> Result<(), String> {
        let data = Data { left: vec![5, 10, 15, 20], right: vec![5, 10, 12, 15] };
        // (5*1) + (10*1) + (15*1) +(20*0) = 5+10+15 => 30
        assert_eq!(30, part2(&data));
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let lines = get_lines(&"input-sample.txt".to_owned());
        let data = part1(lines);
        assert_eq!(31, part2(&data));
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let lines = get_lines(&"input-sample.txt".to_owned());
        let data = part1(lines);
        assert_eq!(11, calc_distances(&data));
        Ok(())
    }

    #[test]
    fn test_negatives() -> Result<(), String> {
        let data = Data { left: vec![5, 10, 15, 20], right: vec![5, 10, 12, 15] };
        // (5-5) + (10-10) + (12-15) + (15-20) = 0+0-3-5 => -8
        assert_eq!(8, calc_distances(&data));
        Ok(())
    }

    #[test]
    fn test_left_sorted() -> Result<(), String> {
        let lines = get_lines(&"input-sample.txt".to_owned());
        let data = part1(lines);
        let mut prev = 0_i32;

        for val in data.left {
            if val >= prev {
                prev = val;
            } else {
                panic!("left array not sorted");
            }
        }

        Ok(())
    }

    #[test]
    fn test_right_sorted() -> Result<(), String> {
        let lines = get_lines(&"input.txt".to_owned());
        let data = part1(lines);
        let mut prev = 0_i32;

        for val in data.right {
            if val >= prev {
                prev = val;
            } else {
                panic!("left array not sorted");
            }
        }

        Ok(())
    }
}