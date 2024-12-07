use aoc_utils::{get_lines, str_to_vec};
use itertools::Itertools;

static DEBUG: bool = false;

#[derive(Clone)]
struct Input {
    final_num: u128,
    values: Vec<u128>
}

fn part1(data: Vec<Input>) -> u128 {
    let ops = vec!["+","*"];
    let mut p1_sum = 0_u128;
    for eval in data {
        let perms = eval.values.len()-1;
        let mut signs: Vec<_> = vec![];
        if perms == 1 {
            signs = vec!["*".to_owned(), "+".to_owned()];
        } else {
            signs = (2..perms).fold(
                ops.iter().cartesian_product(ops.iter()).map(|(&a,&b)| a.to_owned() + b).collect(),
                |acc, _| acc.into_iter().cartesian_product(ops.iter()).map(|(a,b)| a.to_owned() + b).collect()
            );
        }

        if DEBUG { println!("signs: {:?}", signs); }

        let mut all_values = vec![];
        let mut s_idx = 0_usize;
        for s_it in signs {
            if DEBUG { println!("sit: {}", s_it); }
            let mut values = eval.values.clone();
            let sign: Vec<char> = s_it.chars().collect();
            loop {
                if values.len() == 1 { 
                    all_values.push(values[0]); 
                    s_idx = 0;
                    break;
                }
                let first = values.remove(0);
                let second = values.remove(0);
                let s = sign[s_idx];
                if DEBUG { println!("f: {}, s: {}, sign: {}", first, second, s); }
                match s {
                    '+' => {
                        let sum = first+second;
                        values.insert(0, sum);
                    },
                    '*' => {
                        let product = first*second;
                        values.insert(0, product);
                    },
                    _ => { panic!("Invalid sign"); }
                }

                s_idx += 1;
                if s_idx > sign.len()-1 { s_idx = sign.len()-1; }
            }
        }

        if DEBUG { println!("{:?}", all_values); }

        for value in all_values {
            if value == eval.final_num {
                if DEBUG { println!("final num: {}, val: {}", eval.final_num, value); }
                p1_sum += eval.final_num;
                break;
            }
        }
    }

    p1_sum
}

fn part2(data: Vec<Input>) -> u128 {
    let ops = vec!["+","*","X"];
    let mut p1_sum = 0_u128;
    for eval in data {
        let perms = eval.values.len()-1;
        let mut signs: Vec<_> = vec![];
        if perms == 1 {
            signs = vec!["*".to_owned(), "+".to_owned(), "X".to_owned()];
        } else {
            signs = (2..perms).fold(
                ops.iter().cartesian_product(ops.iter()).map(|(&a,&b)| a.to_owned() + b).collect(),
                |acc, _| acc.into_iter().cartesian_product(ops.iter()).map(|(a,b)| a.to_owned() + b).collect()
            );
        }

        if DEBUG { println!("signs: {:?}", signs); }

        let mut all_values = vec![];
        let mut s_idx = 0_usize;
        for s_it in signs {
            if DEBUG { println!("sit: {}", s_it); }
            let mut values = eval.values.clone();
            let sign: Vec<char> = s_it.chars().collect();
            loop {
                if values.len() == 1 { 
                    all_values.push(values[0]); 
                    s_idx = 0;
                    break;
                }
                let first = values.remove(0);
                let second = values.remove(0);
                let s = sign[s_idx];
                if DEBUG { println!("f: {}, s: {}, sign: {}", first, second, s); }
                match s {
                    '+' => {
                        let sum = first+second;
                        values.insert(0, sum);
                    },
                    '*' => {
                        let product = first*second;
                        values.insert(0, product);
                    },
                    'X' => {
                        let combo = (first.to_string()+&second.to_string()).parse::<u128>().expect("NaN");
                        values.insert(0, combo);
                    }
                    _ => { panic!("Invalid sign"); }
                }

                // println!("vals: {:?}", values);
                s_idx += 1;
                if s_idx > sign.len()-1 { s_idx = sign.len()-1; }
            }
        }

        if DEBUG { println!("{:?}", all_values); }

        for value in all_values {
            if value == eval.final_num {
                if DEBUG { println!("final num: {}, val: {}", eval.final_num, value); }
                p1_sum += eval.final_num;
                break;
            }
        }
    }

    p1_sum
}

fn main() {
    let data = build_input(&"input.txt".to_owned());
    println!("p1: {}", part1(data.clone()));
    println!("p2: {}", part2(data));
}

fn build_input(file: &String) -> Vec<Input> {
    let mut input = vec![];
    for l in get_lines(file) {
        let line = l.unwrap();
        let mut split = line.split(":");
        // println!("{:?} -> {:?}", split.next(), split.next().unwrap().trim());
        input.push(Input { final_num: split.next().unwrap().to_string().parse::<u128>().expect("NaN"), values: str_to_vec::<u128>(split.next().unwrap().trim().to_string(), " ")});
    }

    input
}
