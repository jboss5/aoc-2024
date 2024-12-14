use std::collections::HashMap;

use aoc_utils::get_lines;

fn main() {
    let data = build_input(&"input.txt".to_owned());

    let count: u128 = data.iter().map(|stone| {
        let mut cache: HashMap<String,u128> = HashMap::new();
        let o = part1_2(stone.to_string(), 75, &mut cache);
        println!("CACHE SIZE: {}", cache.len());

        o
    }).sum();

    println!("part1: {}", count);
}

fn run(mut input: String) -> Vec<String> {
    if input.eq("0") {
        vec!["1".to_owned()]
    } else if input.len() % 2 == 0 {
        let mut value = String::new();
        for _i in 0..(input.len()/2) {
            let v = input.remove(0);
            value.push(v);
        }

        vec![input.parse::<u128>().expect("NaN").to_string(), value.parse::<u128>().expect("NaN").to_string()]
    } else {
        vec![(input.parse::<u128>().expect("NaN in process") * 2024).to_string()]
    }
}

fn part1_2(stone: String, blinks: u32, cache: &mut HashMap<String,u128>) -> u128 {
    let k = format!("{}||{}",stone,blinks);
    let v = cache.get(&k);
    if v.is_some() {
        return *v.unwrap();
    }

    let l: Vec<String> = run(stone);

    if blinks == 1 {
        let val = l.len() as u128;
        cache.insert(k, val);
        val
    } else {
        let mut o = 0;
        o = part1_2(l[0].clone(), blinks-1, cache);
        if l.len() == 2 {
            o += part1_2(l[1].clone(), blinks-1, cache);
        }

        cache.insert(k, o);
        o
    }
}

fn build_input(file: &String) -> Vec<String> {
    get_lines(file).next().unwrap().expect("error parsing input")
        .split_whitespace()
            .map(|sp| sp.to_string())
            .collect()
}
