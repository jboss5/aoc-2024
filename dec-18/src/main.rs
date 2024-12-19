use std::collections::{BTreeSet, HashMap, HashSet};
use aoc_utils::{get_lines_str, man_dist, Coord};

struct PathFinder {
    grid: Vec<Vec<char>>,
    start: Coord,
    end: Coord,
    visited: HashSet<Coord>,
    cost_map: HashMap<Coord, i64>,
    path_map: HashMap<Coord, Coord>
}

impl PathFinder {
    pub fn new(grid: &Vec<Vec<char>>, start: Coord, end: Coord) -> PathFinder {
        PathFinder { grid: grid.clone(), start, end, visited: HashSet::<Coord>::new(), cost_map: HashMap::new(), path_map: HashMap::new() }
    }

    pub fn get_path_map(self) -> HashMap<Coord, Coord> {
        self.path_map
    }

    fn get_paths(&self, node: Coord) -> Vec<Coord> {
        let mut out = vec![];
        let y = node.y;
        let x = node.x;
        if y > 0 && self.grid[y-1][x] != '#' { out.push(Coord{x, y: y-1}); }
        if x > 0 && self.grid[y][x-1] != '#' { out.push(Coord{x: x-1, y }); }
        if x < self.grid[y].len()-1 && self.grid[y][x+1] != '#' { out.push(Coord{x: x+1, y }); }
        if y < self.grid.len()-1 && self.grid[y+1][x] != '#' { out.push(Coord{x, y: y+1}); }

        out
    }

    pub fn run(&mut self, start: Coord) -> Option<i64> {
        self.path_map.insert(self.start, Coord { y: 0, x: 0 });
        self.cost_map.insert(start, 0);

        let mut queue2 = BTreeSet::new();;
        queue2.insert((0, self.start));

        while !queue2.is_empty() {
            let node = queue2.pop_first().unwrap();

            if node.1 == self.end {
                println!("found end");
                return Some(node.0);
            }

            self.get_paths(node.1).iter().for_each(|n| {
                let priority = man_dist(&self.end, &n);
                let curr = self.cost_map[&node.1]+1;
                let old = *self.cost_map.get(&n).or(Some(&i64::MAX)).unwrap();
                if curr < old {
                    queue2.insert((curr+priority, *n));
                    self.path_map.insert(*n, node.1);
                    self.cost_map.insert(*n, curr);
                }
            });
        }

        println!("bad");
        None
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x]);
        }

        println!();
    }
}

fn update_grid(grid: &mut Vec<Vec<char>>, path_map: &HashMap<Coord, Coord>, start: &Coord, end: &Coord) -> usize {
    let mut node = end;
    let mut cost = 0_usize;
    loop {
        let next_node = path_map.get(&node);
        grid[node.y][node.x] = 'q';
        cost += 1;

        if next_node.is_none() {
            println!("NOT FOUND {:?}", node);
            break;
        }

        if next_node.unwrap() == start || next_node.unwrap() == end {
            grid[next_node.unwrap().y][next_node.unwrap().x] = 'q';
            break;
        }
        node = next_node.unwrap();
    }

    cost
}

fn p1() {
    let blocks = build_input("input.txt");
    let size = 71_usize;
    let mut grid = build_grid(size, &blocks, 1024);
    let start = Coord { y: 0, x: 0 };
    let end = Coord { y: grid.len()-1, x: grid[0].len()-1 };

    print_grid(&grid);

    let mut pathfinder = PathFinder::new(&grid, start, end);
    let t = pathfinder.run(start);
    println!("t: {:?}", t);

    let cost = update_grid(&mut grid, &pathfinder.path_map, &start, &end);

    let mut c2 = 0_usize;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'q' { c2 += 1; }
        }
    }

    println!("\ncost: {cost} c2: {c2} {}", pathfinder.cost_map.get(&end).unwrap());
    print_grid(&grid);
}

fn p2() {
    let blocks = build_input("input.txt");
    let size = 71_usize;
    let start = 1024;
    let mut i = start;
    loop {
        let mut grid = build_grid(size, &blocks, i);
        let start = Coord { y: 0, x: 0 };
        let end = Coord { y: grid.len() - 1, x: grid[0].len() - 1 };

        let mut pathfinder = PathFinder::new(&grid, start, end);
        let has_end = pathfinder.run(start);
        if has_end.is_none() || i > 25000 {
            println!("p2: {:?}", blocks[i-1]);
            return;
        }

        i += 1;
    }
}

fn main() {
    p1();
    p2();
}

fn build_grid(size: usize, blocks: &Vec<Coord>, num_coords: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; size]; size];
    for i in 0..num_coords {
        let coord = blocks[i];
        grid[coord.y][coord.x] = '#';
    }

    grid
}

fn build_input(file: &str) -> Vec<Coord> {
    get_lines_str(file).map(|l| {
            let t = l.unwrap();
            let mut line = t.split(",");
            let x = line.next().unwrap().parse::<usize>().unwrap();
            let y = line.next().unwrap().parse::<usize>().unwrap();
            Coord { y, x }
        }
    ).collect()
}