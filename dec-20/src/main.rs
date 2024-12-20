use rayon::iter::ParallelIterator;
use std::collections::{BTreeSet, HashMap};
use aoc_utils::{get_lines_str, man_dist, Coord};
use rayon::iter::IntoParallelRefIterator;

struct PathFinder {
    start: Coord,
    end: Coord,
    grid: Vec<Vec<char>>,
    cost_map: HashMap<Coord, usize>,
    path_map: HashMap<Coord, Coord>
}

impl PathFinder {
    pub fn new(start: Coord, end: Coord, grid: &Vec<Vec<char>>) -> PathFinder {
        PathFinder { start, end, grid: grid.clone(), cost_map: HashMap::new(), path_map: HashMap::new() }
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

    pub fn get_shortest(&mut self) -> Option<Coord> {
        let mut queue = BTreeSet::new();
        queue.insert((0, self.start));
        self.cost_map.insert(self.start, 0);

        while !queue.is_empty() {
            let current = queue.pop_first().unwrap();

            if current.1 == self.end {
                return Some(current.1);
            }

            self.get_paths(current.1).iter().for_each(|next| {
                let dist = man_dist(&self.end, &next) as usize;
                let cost = self.cost_map[&current.1]+1;
                let old_cost = *self.cost_map.get(&next).or(Some(&usize::MAX)).unwrap();
                if cost < old_cost {
                    queue.insert((cost +dist, *next));
                    self.path_map.insert(*next, current.1);
                    self.cost_map.insert(*next, cost);
                }
            });
        }

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

fn fill_path(grid: &mut Vec<Vec<char>>, end: &Coord, start: &Coord, path_map: HashMap<Coord, Coord>) -> i32 {
    let mut current = start;
    let mut count = 0;
    while current != end {
        grid[current.y][current.x] = 'x';
        let next = path_map.get(&current);

        // count +=1 ;
        if next.is_none() {
            break;
        }
        current = next.unwrap();
    }

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'x' { count +=1; }
        }
    }

    count
}

fn main() {
    let mut orig_grid = build_input("input-sample.txt");
    let start_end = get_start_end(&orig_grid);
    orig_grid[start_end.0.y][start_end.0.x] = 'S';
    orig_grid[start_end.1.y][start_end.1.x] = 'E';
    let blocks = start_end.2;
    let mut orig_pf = PathFinder::new(start_end.0, start_end.1, &orig_grid);
    orig_pf.get_shortest();
    let base_count = fill_path(&mut orig_grid.clone(), &start_end.0, &start_end.1, orig_pf.path_map);
    // let mut count_map = HashMap::new();

    blocks.par_iter().for_each(|b| {
        let mut grid = orig_grid.clone();

        grid[b.y][b.x] = '.';

        let mut pf = PathFinder::new(start_end.0, start_end.1, &grid);
        if pf.get_shortest().is_some() {
            let count = fill_path(&mut grid, &start_end.0, &start_end.1, pf.path_map);
            let saved = base_count - count;
            println!("Saved {saved}");

            println!("b {:?} saved: {saved}, base {base_count} new {count}", b);
            print_grid(&grid);
            println!();
        }
    });

    // get_lines_str("output.txt").for_each(|l| {
    //     let line = l.unwrap();
    //     let mut split = line.split(" ");
    //     split.next();
    //     let saved = split.next().unwrap().parse::<usize>().unwrap();
    //     count_map.entry(saved).and_modify(|v| *v += 1).or_insert(1usize);
    // });
    //
    // let c = count_map.iter().filter(|e| (*e.0 != 0) && (*e.0 >= 100)).map(|(k, v)| *v).collect::<Vec<usize>>().into_iter().sum::<usize>();
    // println!("part1 {}", c);
}

fn get_start_end(grid: &Vec<Vec<char>>) -> (Coord, Coord, Vec<Coord>) {
    let mut start = Coord { x: 0, y: 0 };
    let mut end = Coord { x: 0, y: 0 };
    let mut block_vec = vec![];

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            match grid[y][x] {
                'S' => start = Coord { y, x},
                'E' => end = Coord { y, x},
                '#' => {
                    if y == 0 || x == 0 || y == grid.len()-1 || x == grid[0].len()-1 { continue; }
                    block_vec.push(Coord{x, y})
                },
                _ => {}
            }
        }
    }

    (start, end, block_vec)
}

fn build_input(file: &str) -> Vec<Vec<char>> {
    get_lines_str(file).map(|l| {
        l.unwrap().chars().collect()
    }).collect()
}