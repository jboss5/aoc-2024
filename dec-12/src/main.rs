use std::collections::{HashMap, HashSet};

use aoc_utils::{get_lines, Coord};

#[derive(Debug)]
struct Region {
    id: char,
    plots: HashSet<Coord>
}

fn main() {
    let data = build_input(&"input.txt".to_owned());
    let output = part1(data.clone());

    let p1: u32 = output.iter().map(|r| calc_region(r, &data)).sum();
    let p2: u32 = output.iter().map(|r| calc_region_p2(r, &data)).sum();

    println!("output: {:?}", output);
    println!("part1: {}", p1);
    println!("part2: {}", p2);
}

fn calc_region_p2(region: &Region, data: &Vec<Vec<char>>) -> u32 {
    println!("p2 region {} size {}", region.id, region.plots.len());
    let id = region.id;
    let mut outside_hit_2 = HashMap::new();
    for plot in region.plots.iter() {

        if plot.y == 4 && plot.x == 0 {
            println!("on e4");
        }

        // north
        if plot.y == 0 || data[plot.y-1][plot.x] != id{
            let k = "n".to_owned()+&plot.y.to_string();
            outside_hit_2.entry(k).or_insert(vec![]).push(Coord { y: plot.y, x: plot.x });
        } 

        // south
        if plot.y == data.len() - 1 || data[plot.y+1][plot.x] != id {
            let k = "s".to_owned()+&plot.y.to_string();
            outside_hit_2.entry(k).or_insert(vec![]).push(Coord { y: plot.y, x: plot.x });
        } 

        // east
        if plot.x == data[plot.y].len() - 1 || data[plot.y][plot.x+1] != id {
            let k = "e".to_owned()+&plot.x.to_string();
            outside_hit_2.entry(k).or_insert(vec![]).push(Coord { y: plot.y, x: plot.x });
        } 

        // west
        if plot.x == 0 || data[plot.y][plot.x-1] != id {
            let k = "w".to_owned()+&plot.x.to_string();
            outside_hit_2.entry(k).or_insert(vec![]).push(Coord { y: plot.y, x: plot.x });
        } 
    }

    let mut size = 0_u32;
    for e in outside_hit_2.iter() {
        if e.0.starts_with("w") || e.0.starts_with("e") {
            // x same
            let mut vec = e.1.clone();
            vec.sort_by(|a,b| a.y.cmp(&b.y));
            vec.windows(2).for_each(|q| {
                let y1 = q[0].y as i32;
                let y2 = q[1].y as i32;
                if y1+1 != y2 {
                    size +=1;
                }
            });
        } else if e.0.starts_with("s") || e.0.starts_with("n") {
            // y same
            let mut vec = e.1.clone();
            vec.sort_by(|a,b| a.x.cmp(&b.x));
            vec.windows(2).for_each(|q| {
                let x1 = q[0].x as i32;
                let x2 = q[1].x as i32;
                if x1+1 != x2 {
                    size +=1;
                }
            });
        }
    }

    size += outside_hit_2.len() as u32;
    region.plots.len() as u32 * size
}

fn calc_region(region: &Region, data: &Vec<Vec<char>>) -> u32 {
    println!("region {} size {}", region.id, region.plots.len());
    let id = region.id;
    let mut hit = vec![];
    for plot in region.plots.iter() {

        // north
        if plot.y == 0 {
            hit.push(plot.clone());
        } else if data[plot.y-1][plot.x] != id {
            hit.push(Coord { y: plot.y-1, x: plot.x });
        }

        // south
        if plot.y >= data.len() - 1 {
            hit.push(plot.clone());
        } else if data[plot.y+1][plot.x] != id {
            hit.push(Coord { y: plot.y+1, x: plot.x });
        }

        // east
        if plot.x >= data[plot.y].len() - 1 {
            hit.push(plot.clone());
        } else if data[plot.y][plot.x+1] != id {
            hit.push(Coord { y: plot.y, x: plot.x+1 });
        }

        // west
        if plot.x == 0 {
            hit.push(plot.clone());
        } else if data[plot.y][plot.x-1] != id {
            hit.push(Coord { y: plot.y, x: plot.x-1 });
        }
    }

    println!("hit, size: {}: {:?}", hit.len(), hit);
    region.plots.len() as u32 * hit.len() as u32
}

fn get_plots(id: char, data: &Vec<Vec<char>>, y: usize, x: usize, visited: &mut HashSet<Coord>) {
    let current = data[y][x];
    let cur_coord = Coord { y, x }; 

    if current != id || visited.contains(&cur_coord) { 
        return;
    } else {
        visited.insert(cur_coord);
    }

    // north
    if y > 0 {
        get_plots(id, data, y-1, x, visited);
    }

    // south
    if y < data.len()-1 {
        get_plots(id, data, y+1, x, visited);
    }

    // east
    if x < data[y].len()-1 {
        get_plots(id, data, y, x+1, visited);
    }

    // west
    if x > 0 {
        get_plots(id, data, y, x-1, visited);
    }
}

fn part1(data: Vec<Vec<char>>) -> Vec<Region> {
    let mut regions = vec![];
    let mut mut_data = data.clone();
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            let id = mut_data[y][x];
            if id == '.' { continue; }

            let mut plots = HashSet::<Coord>::new();
            get_plots(id, &data, y, x, &mut plots);
            for p in plots.iter() {
                mut_data[p.y][p.x] = '.';
            }
            
            regions.push(Region { id, plots });
        }
    }
    
    regions
}

fn build_input(file: &String) -> Vec<Vec<char>> {
    get_lines(file).map(|l| 
        l.unwrap().chars().collect()
    ).collect()
}
