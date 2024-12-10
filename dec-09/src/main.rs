use std::collections::{BTreeMap, HashMap};

use aoc_utils::get_lines;

static DEBUG: bool = false;

fn main() {
    let data = build_input(&"input.txt".to_owned());
    if DEBUG { println!("{:?}", data); }
    println!("part1: {}", part1(data.clone()));
    println!("part2: {}", part2(data));
}

fn part1(data: (Vec<String>, Vec<usize>, HashMap<usize, Vec<usize>>, BTreeMap<usize, u32>)) -> u128 {
    let input = data.0;
    let mut empty_vec = data.1;
    let mut out = 0_u128;
    let mut block_map = input.clone();
    empty_vec.reverse();

    for _ in 0..empty_vec.len() {
        let str = block_map.pop().expect("No char found");
        if str.eq(".") { continue; }
        block_map[empty_vec.pop().expect("No usize found")] = str;
    }

    if DEBUG { println!("blocks: {:?}", block_map); }

    for i in 0..block_map.len() {
        let val = &block_map[i];
        if val.eq(".") { break; }
        out += i as u128 * val.parse::<u128>().expect("NaN");
    }

    out
}

fn part2(data: (Vec<String>, Vec<usize>, HashMap<usize, Vec<usize>>, BTreeMap<usize, u32>)) -> u128 {
    let mut input = data.0;
    let mut empty_vec = data.1;
    let mut empty_size_map = data.3;
    let occ_map = BTreeMap::from_iter(data.2.iter());
    let mut out = 0_u128;
    empty_vec.reverse();


    for check_entry in occ_map.iter().rev() {
        if DEBUG { println!("checking: {}", check_entry.0); }
        let size = check_entry.1.len();
        let mut map = empty_size_map.clone();
        let occ = occ_map.get(check_entry.0).expect("Not found");
        if DEBUG { println!("occ: {:?}", occ); }

        for e_empty in map.iter_mut() {
            if e_empty.0 > &occ[0] { continue; }

            if *e_empty.1 > 0 && size <= *e_empty.1 as usize {
                let mut k = 0_usize;
                let start = e_empty.0.clone();
                let mut new_start = e_empty.0.clone();
                for j in start..(start+check_entry.1.len()) {
                    input[j] = check_entry.0.to_string();
                    input[check_entry.1[k]] = ".".to_owned();
                    k += 1;
                    *e_empty.1 -= 1;
                    new_start += 1;
                }

                empty_size_map.remove(e_empty.0);
                empty_size_map.insert(new_start, *e_empty.1);

                break;
            }
        }
    }

    if DEBUG { println!("p2 map: {:?}", input); }

    for i in 0..input.len() {
        let val = &input[i];
        if val.eq(".") { continue; }
        out += i as u128 * val.parse::<u128>().expect("NaN");
    }

    out
}


fn build_input(file: &String) -> (Vec<String>, Vec<usize>, HashMap<usize, Vec<usize>>, BTreeMap<usize, u32>) {
    let raw_input: Vec<u32> = get_lines(file).next()
        .unwrap().expect("Error parsing input")
        .chars()
            .map(|ch| ch.to_digit(10).expect("NaN"))
            .collect();
    
    let mut out = vec![];
    let mut id = 0_usize;
    let mut digit_cnt = 0_usize;
    let mut empty_vec = vec![];
    let mut occ_map = HashMap::new();
    let mut empty_size_map = BTreeMap::new();

    while id < raw_input.len() {
        (0..raw_input[id]).for_each(|_| {
            let digit_as_str = digit_cnt.to_string();
            out.push(digit_as_str.clone());
            occ_map.entry(digit_cnt).or_insert(vec![]).push(out.len()-1 as usize);
        });
        
        if id == raw_input.len()-1 { break; }
        let start = out.len();
        (0..raw_input[id+1]).for_each(|_| {
            out.push(".".to_owned());
            empty_vec.push(out.len()-1);
        });

        empty_size_map.insert(start, raw_input[id+1]);

        id += 2;
        digit_cnt += 1;
    }

    (out, empty_vec, occ_map, empty_size_map)
}

