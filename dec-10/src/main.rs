use std::collections::HashSet;

use aoc_utils::{get_lines, Coord};

static DEBUG: bool = false;

struct Input {
    grid: Vec<Vec<usize>>,
    start_list: Vec<Coord>
}

fn main() {
    let data = build_input(&"input.txt".to_owned());
    if DEBUG { println!("data: {:?}", data.grid); }
    
    let mut total_nines = 0;
    let mut p2_total = 0;
    for zero in data.start_list {
        let mut found = Vec::<Coord>::new();
        part1(&data.grid, zero.y, zero.x, &mut found);
        if DEBUG { println!("found: {:?}", found); }
        p2_total += found.len();

        let mut temp = HashSet::new();
        found.iter().for_each(|v| { temp.insert(*v); });
        total_nines += temp.len();
    }

    println!("part1: {}", total_nines);
    println!("part2: {}", p2_total);
}

fn get_neighbors(grid: &Vec<Vec<usize>>, y: usize, x: usize) -> Vec<Coord> {
    let curr = grid[y][x];
    let next = curr+1;
    let mut out = vec![];

    //north
    if y > 0 && grid[y-1][x] == next {
        out.push(Coord { y: y-1, x });
    }

    //south
    if y < grid.len()-1 && grid[y+1][x] == next {
        out.push(Coord { y: y+1, x });
    }

    //east
    if x < grid[y].len()-1 && grid[y][x+1] == next {
        out.push(Coord { y, x: x+1 });
    }

    //west
    if x > 0 && grid[y][x-1] == next {
        out.push(Coord { y, x: x-1 });
    }
    
    out
}

fn part1(grid: &Vec<Vec<usize>>, start_y: usize, start_x: usize, found_nines: &mut Vec<Coord>) {
    if DEBUG { println!("VALUE: {} checking: ({},{})", grid[start_y][start_x], start_y, start_x); }
    if grid[start_y][start_x] == 9 {
        found_nines.push(Coord { y: start_y, x: start_x });
        return;
    }

    for n in get_neighbors(grid, start_y, start_x) {
        part1(grid, n.y, n.x, found_nines);
    }
}

fn build_input(file: &String) -> Input {
    let grid: Vec<Vec<usize>> = get_lines(file).map(|l| l
        .unwrap()
        .chars()
            .map(|ch| 
                ch
                .to_digit(10)
                .expect("NaN") as usize
            ).collect()
    ).collect();

    let mut start_list = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 { start_list.push(Coord { y, x }); }
        }
    }

    Input { grid, start_list }
}