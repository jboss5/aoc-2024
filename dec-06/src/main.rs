use std::collections::HashSet;

use aoc_utils::get_lines;

static DEBUG: bool = false;

#[derive(Clone,PartialEq,Debug,Eq,Hash)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

#[derive(Clone)]
struct Grid {
    x: usize,
    y: usize,
    path: Vec<Vec<char>>,
    visited: HashSet<(usize,usize,Direction)>
}

fn main() {
    let data = build_input(&"input.txt".to_owned());
    println!("Part 1: {}", part1(data.clone()));
    println!("Part 2: {}", part2(data));
}

fn build_input(file: &String) -> Grid {
    let mut out = vec![];
    let mut y = 0_usize;
    let mut char_x = 0_usize;
    let mut found = false;

    for l in get_lines(file) {
        let chars: Vec<char> = l.unwrap().chars().collect();
        for i in 0..chars.len() {
            if chars[i] == '^' { 
                char_x = i; 
                found = true;
                break;
            }
        }
        out.push(chars);

        if !found { y += 1; }
    }

    Grid { x: char_x, y, path: out, visited: HashSet::new() }
}

fn get_incr(visited: bool) -> u32 {
    match visited {
        true => 0,
        false => 1,
    }
}

fn move_it2(current: (usize,usize), g: &mut Grid, direction: &Direction) -> u32 {
    let y = current.0;
    let x = current.1;
    let grid = &mut g.path;

    // if need a multiple turn situation, backtrack then go the other way
    if grid[y][x] == '#' {
        match direction {
            Direction::NORTH => return move_it2((y+1,x), g, &Direction::EAST),
            Direction::SOUTH => return move_it2((y-1,x), g, &Direction::WEST),
            Direction::EAST => return move_it2((y,x-1), g, &Direction::SOUTH),
            Direction::WEST => return move_it2((y,x+1), g, &Direction::NORTH),
        }
    }

    let visited = grid[y][x] == 'X';
    if g.visited.contains(&(y,x,direction.clone())) {
        if DEBUG { println!("infa loop: ({},{})",y,x); }
        grid[y][x] = 'O';
        return 1;
    } else {
        g.visited.insert((y,x,direction.clone()));
    }
    if !visited { 
        grid[y][x] = 'X'; 
    }
    if DEBUG { println!("current: {},{} -- visited: {}", y,x,visited); }

    match direction {
        Direction::NORTH => {
            let mut new_direction = Direction::NORTH;
            if y == 0 { return 0_u32; }
            else if grid[y-1][x] == '#' {
                new_direction = Direction::EAST;
                return move_it2((y,x+1), g, &new_direction);
            } else {
                return move_it2((y-1,x), g, &new_direction);
            }
        },
        Direction::SOUTH => {
            let mut new_direction = Direction::SOUTH;
            if y == grid.len()-1 { return 0_u32; }
            else if grid[y+1][x] == '#' {
                new_direction = Direction::WEST;
                return move_it2((y,x-1), g, &new_direction);
            } else {
                return move_it2((y+1,x), g, &new_direction);
            }
        },
        Direction::EAST => {
            let mut new_direction = Direction::EAST;
            if x == grid[y].len()-1 { return 0_u32; }
            else if grid[y][x+1] == '#' {
                new_direction = Direction::SOUTH;
                return move_it2((y+1,x), g, &new_direction);
            } else {
                return move_it2((y,x+1), g, &new_direction);
            }
        },
        Direction::WEST => {
            let mut new_direction = Direction::WEST;
            if x == 0 { return 0_u32; }
            else if grid[y][x-1] == '#' {
                new_direction = Direction::NORTH;
                return move_it2((y-1,x), g, &new_direction);
            } else {
                return move_it2((y,x-1), g, &new_direction);
            }
        },
    }
}

fn move_it(current: (usize,usize), g: &mut Grid, direction: &Direction) -> u32 {
    let y = current.0;
    let x = current.1;
    let grid = &mut g.path;

    let visited = grid[y][x] == 'X';
    if !visited { 
        grid[y][x] = 'X'; 
    }
    if DEBUG { println!("current: {},{} -- visited: {}", y,x,visited); }

    match direction {
        Direction::NORTH => {
            let mut new_direction = Direction::NORTH;
            if y == 0 { return 1_u32; }
            else if grid[y-1][x] == '#' {
                new_direction = Direction::EAST;
                return get_incr(visited) + move_it((y,x+1), g, &new_direction);
            } else {
                return get_incr(visited) + move_it((y-1,x), g, &new_direction);
            }
        },
        Direction::SOUTH => {
            let mut new_direction = Direction::SOUTH;
            if y == grid.len()-1 { return 1_u32; }
            else if grid[y+1][x] == '#' {
                new_direction = Direction::WEST;
                return get_incr(visited) + move_it((y,x-1), g, &new_direction);
            } else {
                return get_incr(visited) + move_it((y+1,x), g, &new_direction);
            }
        },
        Direction::EAST => {
            let mut new_direction = Direction::EAST;
            if x == grid[y].len()-1 { return 1_u32; }
            else if grid[y][x+1] == '#' {
                new_direction = Direction::SOUTH;
                return get_incr(visited) + move_it((y+1,x), g, &new_direction);
            } else {
                return get_incr(visited) + move_it((y,x+1), g, &new_direction);
            }
        },
        Direction::WEST => {
            let mut new_direction = Direction::WEST;
            if x == 0 { return 1_u32; }
            else if grid[y][x-1] == '#' {
                new_direction = Direction::NORTH;
                return get_incr(visited) + move_it((y-1,x), g, &new_direction);
            } else {
                return get_incr(visited) + move_it((y,x-1), g, &new_direction);
            }
        },
    }
}

fn part1(mut data: Grid) -> u32 {
    let r = move_it((data.y,data.x), &mut data, &Direction::NORTH);
    if DEBUG {
        for v in data.path {
            println!("{:?}", v);
        }
    }

    r
}

fn part2(data: Grid) -> u32 {
    if DEBUG { println!("data: {},{}", data.y, data.x); }
    let mut v = 0_u32;
    for i in 0..data.path.len() {
        for k in 0..data.path[i].len() {
            let mut grid2 = data.clone();
            grid2.path[i][k] = '#';
            let ret = move_it2((data.y,data.x), &mut grid2, &Direction::NORTH);
            v += ret;

            if false && ret > 0 {
                println!("grid:");
                for v in &grid2.path {
                    for y in v {
                        print!("{}",y);
                    }
        
                    println!();
                }
                println!();
            }
        }
    }

    v
}
