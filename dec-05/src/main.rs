use std::{collections::HashMap, fs::File, io::{BufReader, Lines}};
use aoc_utils::{get_lines, str_to_vec};

struct Input {
    sort_order: HashMap<u32, Vec<u32>>,
    page_updates: Vec<Vec<u32>>
}

fn build_input(lines: Lines<BufReader<File>>) -> Input {
    let mut input = Input { sort_order: HashMap::new(), page_updates: vec![] };

    for l in lines {
        let line = l.unwrap();
        if line.contains('|') {
            let split = str_to_vec::<u32>(line, "|");

            input.sort_order
                .entry(split[0])
                .or_insert(Vec::new())
                .push(split[1]);
        } else if line.contains(',') {
            input.page_updates.push(str_to_vec::<u32>(line, ","));
        }
    }

    input
}

fn main() {
    let input = &build_input(get_lines(&"input.txt".to_owned()));
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn is_sorted(vec: &Vec<u32>, input: &Input) -> bool {
    for w in vec.windows(2) {
        if input.sort_order.contains_key(&w[0]) {
            if !input.sort_order.get(&w[0]).expect("").contains(&w[1]) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn is_sorted_fix(vec: &Vec<u32>, input: &Input) -> (bool,Vec<u32>) {
    let mut new_vec = vec.clone();
    let mut good = true;
    for i in 0..new_vec.len()-1 {
        let current = new_vec[i];
        let next = new_vec[i+1];
        if input.sort_order.contains_key(&current) {
            let order = input.sort_order.get(&current).expect("");
            if !order.contains(&next) {
                new_vec.swap(i, i+1);
                good = false;
            }
        } else {
            new_vec.swap(i, i+1);
            good = false;
        }
    }

    if !good {
        let mut done = false;
        while !done {
            let ans = is_sorted_fix(&new_vec, input);
            new_vec = ans.1;
            done = ans.0;            
        }

        return (false, new_vec);
    }

    (good, new_vec)
}

fn part1(input: &Input) -> u32 {
    let mut correct = vec![];

    for run in input.page_updates.iter() {
        if is_sorted(run, input) {
            correct.push(run.clone());
        }
    }

    correct.iter().map(|v| v[v.len()/2]).sum()
}

fn part2(input: &Input) -> u32 {
    let mut fixed = vec![];

    for run in input.page_updates.iter() {
        let ans = is_sorted_fix(run, input);
        if !ans.0 {
            fixed.push(ans.1);
        }
    }

    fixed.iter().map(|v| v[v.len()/2]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = get_lines(&"input-sample.txt".to_owned());
        assert_eq!(143, part1(&build_input(input)));
    }

    #[test]
    fn test_part2() {
        let input = get_lines(&"input-sample.txt".to_owned());
        assert_eq!(123, part2(&build_input(input)));
    }
}