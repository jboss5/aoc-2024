
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
pub struct Coord {
    x: usize,
    y: usize
}

pub fn get_lines(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(filename).expect("error opening file")).lines()
}

pub fn man_dist(coord1: &Coord, coord2: &Coord) -> i64 {
    i64::abs(((coord1.x - coord2.x) + (coord1.y - coord2.y)) as i64)
}
