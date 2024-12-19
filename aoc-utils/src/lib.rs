
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;


#[derive(Debug,Copy,Clone,Eq,Ord,PartialOrd)]
pub struct Coord {
    pub x: usize,
    pub y: usize
}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x + self.y).hash(state);
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("error opening file")).lines()
}

pub fn get_lines_str(filename: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("error opening file")).lines()
}

pub fn man_dist(coord1: &Coord, coord2: &Coord) -> i64 {
    (coord1.x as i64 - coord2.x as i64).abs() + (coord1.y as i64 - coord2.y as i64).abs()
}

pub fn str_to_vec<T: FromStr>(line: String, delim: &str) -> Vec<T> {
    Vec::from_iter(line.split(delim)
        .map(|s| T::from_str(s).ok().unwrap())
    )
}
