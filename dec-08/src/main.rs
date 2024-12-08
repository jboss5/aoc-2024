use std::collections::{HashMap, HashSet, VecDeque};

use aoc_utils::{get_lines, Coord};

static DEBUG: bool = false;

#[derive(Clone)]
struct Input {
    ants: HashMap<char, VecDeque<Coord>>,
    grid: Vec<Vec<char>>
}

fn print_input_grid(grid: &Vec<Vec<char>>) {
    if !DEBUG { return; }

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x]);
        }

        println!();
    }
}

fn main() {
    let mut input = build_input(&"input.txt".to_owned());
    println!("part1: {}", part1(&mut input.clone()));
    println!("part2: {}", part2(&mut input));
}

fn build_all_combos(coords: &VecDeque<Coord>) -> Vec<(&Coord,&Coord)> {
    let mut out = vec![];
    for i in 0..coords.len() {
        for k in i+1..coords.len() {
            out.push((coords.get(i).expect("i not found"), coords.get(k).expect("k not found")));
        }
    }

    out
}

fn part2(mut input: &mut Input) -> i32 {

    let grid = &mut input.grid;
    let mut antinodes = HashSet::new();
    print_input_grid(&grid);
    
    for entry in input.ants.iter() {
        entry.1.iter().for_each(|v| { antinodes.insert(*v); });
        let combos = build_all_combos(entry.1);
        if DEBUG { println!("COMBO COORDS {:?}", combos.clone()); }

        for combo in combos {
            let mut start_coord = Coord { x: 0, y: 0};
            let mut end_coord = Coord {x: 0, y: 0};

            if combo.0.x+combo.0.y < combo.1.x+combo.1.y {
                start_coord = *combo.0;
                end_coord = *combo.1;
            } else {
                start_coord = *combo.1;
                end_coord = *combo.0;
            }

            let x_slope = end_coord.x as i32 - start_coord.x as i32;
            let y_slope = end_coord.y as i32 - start_coord.y as i32;

            if DEBUG { println!("p2 start: {:?}, end: {:?}, xslope: {}, yslope: {}", start_coord, end_coord, x_slope, y_slope); }

            loop {
                let new_start = (start_coord.y as i32 - y_slope, start_coord.x as i32 - x_slope);

                if new_start.0 >= 0 && new_start.0 < grid.len() as i32 && new_start.1 >= 0 && new_start.1 < grid[0].len() as i32 {
                    start_coord = Coord { y: new_start.0 as usize, x: new_start.1 as usize };
                    antinodes.insert(start_coord);
                    grid[start_coord.y][start_coord.x] = '#';
                } else { break; }
            }

            loop {
                
                let new_end = (end_coord.y as i32 + y_slope, end_coord.x as i32 + x_slope);

                if new_end.0 >= 0 && new_end.0 < grid.len() as i32 && new_end.1 >= 0 && new_end.1 < grid[0].len() as i32 {
                    end_coord = Coord { y: new_end.0 as usize, x: new_end.1 as usize };
                    antinodes.insert(end_coord);
                    grid[end_coord.y][end_coord.x] = '#';
                } else { break; }
            }
        }
    }

    print_input_grid(grid);
    if DEBUG { antinodes.iter().for_each(|n| println!("p2 anti: {:?}", n)); }
    antinodes.len() as i32
}

fn part1(mut input: &mut Input) -> i32 {

    let grid = &mut input.grid;
    let mut antinodes = HashSet::new();
    print_input_grid(&grid);
    
    for entry in input.ants.iter() {
        let combos = build_all_combos(entry.1);

        for combo in combos {
            let mut start_coord = Coord { x: 0, y: 0};
            let mut end_coord = Coord {x: 0, y: 0};

            if combo.0.x+combo.0.y < combo.1.x+combo.1.y {
                start_coord = *combo.0;
                end_coord = *combo.1;
            } else {
                start_coord = *combo.1;
                end_coord = *combo.0;
            }

            let x_slope = end_coord.x as i32 - start_coord.x as i32;
            let y_slope = end_coord.y as i32 - start_coord.y as i32;

            let new_start = (start_coord.y as i32 - y_slope, start_coord.x as i32 - x_slope);
            let new_end = (end_coord.y as i32 + y_slope, end_coord.x as i32 + x_slope);

            if new_start.0 >= 0 && new_start.0 < grid.len() as i32 && new_start.1 >= 0 && new_start.1 < grid[0].len() as i32 {
                let new_coord = Coord { y: new_start.0 as usize, x: new_start.1 as usize };
                antinodes.insert(new_coord);
                grid[new_coord.y][new_coord.x] = '#';
            }

            if new_end.0 >= 0 && new_end.0 < grid.len() as i32 && new_end.1 >= 0 && new_end.1 < grid[0].len() as i32 {
                let new_coord = Coord { y: new_end.0 as usize, x: new_end.1 as usize };
                antinodes.insert(new_coord);
                grid[new_coord.y][new_coord.x] = '#';
            }
        }
    }

    antinodes.len() as i32
}

fn build_input(file: &String) -> Input {
    let mut out: HashMap<char, VecDeque<Coord>> = HashMap::new();
    let mut grid = vec![];
    let mut y = 0_usize;
    for l in get_lines(file) {
        let line = l.unwrap();
        let chars: Vec<char> = line.chars().collect();
        let mut x_grid = vec![];
        for x in 0..chars.len() {
            let ch = chars[x];
            x_grid.push(ch);
            if ch != '.' {
                let coord = Coord { x, y };
                match out.get_mut(&ch) {
                    Some(v) => {
                        if !v.contains(&coord) { v.push_back(coord); }
                    }, None => {
                        let mut v = VecDeque::new();
                        v.push_back(coord);
                        out.insert(ch, v);
                    }
                }
            }
        }

        grid.push(x_grid);

        y += 1;
    }

    Input { ants: out, grid }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut input = build_input(&"input-sample.txt".to_owned());
        assert_eq!(14, part1(&mut input));
    }

    #[test]
    fn test_part2() {
        let mut input = build_input(&"input-sample.txt".to_owned());
        assert_eq!(34, part1(&mut input));
    }
}