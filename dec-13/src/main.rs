use aoc_utils::get_lines;
use regex::Regex;
use rust_decimal::{prelude::{FromPrimitive, ToPrimitive}, Decimal};

#[derive(Debug)]
struct Input {
    a_x: usize,
    a_y: usize,
    b_x: usize,
    b_y: usize,
    x: usize,
    y: usize
}

fn main() {
    let data = build_input(&"input.txt".to_owned());
    println!("won1: {:?}", part1(&data).iter().sum::<i32>());
    println!("won2: {:?}", part2(data).iter().sum::<u128>());
}

fn part1(data: &Vec<Input>) -> Vec<i32> {
    let mut won_prizes = vec![];
    for input in data {
        let mut calc = -1;
        for a in 0..100 {
            let b_prd = (a as f32 * input.a_x as f32 - input.x as f32).abs() / input.b_x as f32;
            if b_prd.fract() == 0.0 {
                let x_eq = input.a_x as i32 * a as i32 + input.b_x as i32 * b_prd as i32;
                let y_eq = input.a_y as i32 * a as i32 + input.b_y as i32 * b_prd as i32;
                if x_eq == input.x as i32 && y_eq == input.y as i32 {
                    calc = a as i32 * 3 + b_prd as i32 * 1;
                    break;
                }
            }
        }
        
        if calc > 0 {
            won_prizes.push(calc);
        }
    }

    won_prizes
}

fn part2(data: Vec<Input>) -> Vec<u128> {
    let mut won_prizes = vec![];
    for mut input in data {
        input.x += 10000000000000;
        input.y += 10000000000000;

        let mut a= input.a_x as f64 - input.a_y as f64;
        let b0 = input.b_x as f64 - input.b_y as f64;
        let end = input.x as f64 - input.y as f64;

        let mut mid = 0.0;
        let mut mid2 = 0.0;
        let mut b = 0.0;
        if a == 0.0 {
            b = (end / b0);
        } else {
            mid = (end / a);
            mid2 = (b0 / (-1.0 * a));
            let m = input.a_x as f64 * mid;
            let m01 = (mid2 * input.a_x as f64);
            let mut m2 = 0.0;
            m2 = input.b_x as f64 + m01;
            let mut f1 = 0.0;
            f1 = input.x as f64 - m;
            b = f1 / m2;
        }

        let a1 = (input.x as f64 - (input.b_x as f64 * b)) / input.a_x as f64;
        let d1 = Decimal::from_str_exact(&a1.to_string());
        let d2 = Decimal::from_str_exact(&b.to_string());
        if d1.is_err() || d2.is_err() {
            continue;
        }

        let mut d3 = d1.unwrap();
        let mut d4 = d2.unwrap();

        if d3.round_dp(2).fract() == Decimal::from_f64(0.00000).unwrap() {
            if d4.round_dp(2).fract() == Decimal::from_f64(0.00000).unwrap() {
                let mut b_128 = 0.0;
                let mut a_128 = 0.0;

                if b.fract() < 0.1 { b_128 = b.floor(); }
                else { b_128 = b.ceil(); }

                if a1.fract() < 0.1 { a_128 = a1.floor(); }
                else { a_128 = a1.ceil(); }

                if (input.a_x as u128 * a_128.to_u128().unwrap()) + (input.b_x as u128 * b_128.to_u128().unwrap()) == input.x as u128 
                    && (input.a_y as u128 * a_128.to_u128().unwrap()) + (input.b_y as u128 * b_128.to_u128().unwrap()) == input.y as u128 {
                        won_prizes.push(((a_128.to_u128().unwrap() * 3)+(b_128.to_u128().unwrap() * 1)));
                        continue;
                }
            }
        }
    }

    won_prizes
}

fn build_input(file: &String) -> Vec<Input> {
    let regex_xy = Regex::new(r"X\+(?<abx>\d+)\, Y\+(?<aby>\d+)").expect("Error building xy regex");
    let regex_prize = Regex::new(r"X=(?<x>\d+)\, Y=(?<y>\d+)").expect("Error building xy regex");
    let mut output = vec![];
    let mut line_vec: Vec<String> = vec![];
    for l in get_lines(file) {
        let s = l.unwrap().to_string();
        if s.is_empty() { continue; }

        if s.starts_with("Prize:") {
            let a_caps = regex_xy.captures(&line_vec[0]).unwrap();
            let b_caps = regex_xy.captures(&line_vec[1]).unwrap();
            let prize_caps = regex_prize.captures(&s).unwrap();

            output.push(Input { 
                a_x: a_caps["abx"].parse::<usize>().expect("ax NaN"), 
                a_y: a_caps["aby"].parse::<usize>().expect("ay NaN"), 
                b_x: b_caps["abx"].parse::<usize>().expect("bx NaN"), 
                b_y: b_caps["aby"].parse::<usize>().expect("by NaN"), 
                x: prize_caps["x"].parse::<usize>().expect("px NaN"), 
                y: prize_caps["y"].parse::<usize>().expect("py NaN")
            });

            line_vec.clear();
        } else {
            line_vec.push(s.to_string());
        }
    }

    output
}