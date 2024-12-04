use aoc_utils::get_lines;

static DEBUG: bool = true;

fn main() {
    let data = build_input(&"input.txt".to_owned());
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn build_input(file: &String) -> Vec<Vec<char>> {
    get_lines(file).map(|l| 
        l.unwrap()
            .chars()
            .collect())
        .collect()
}

fn find_mas(y: usize, x: usize, data: &Vec<Vec<char>>) -> u32 {
    let mut xmas_list: Vec<String> = vec![];

    //NW SE
    if (y >= 1 && x >= 1) && (y <= data.len()-2 && x <= data[y].len()-2) {
        xmas_list.push(vec![data[y-1][x-1],data[y][x],data[y+1][x+1]].iter().collect());
    }
    //NE SW
    if (y >= 1 && x <= data[y].len()-2) && (y <= data.len()-2 && x >= 1) {
        xmas_list.push(vec![data[y-1][x+1],data[y][x],data[y+1][x-1]].iter().collect());
    }

    let mut is_sam = true;
    for str in &xmas_list {
        if DEBUG { println!("p2: {}, ({},{})", str, x, y); }
        if !(str.eq(&"MAS") || str.eq(&"SAM")) { is_sam = false; }
    }

    if is_sam && xmas_list.len() == 2_usize { 1 } else { 0 }
}

fn find_xmas(y: usize, x: usize, data: &Vec<Vec<char>>) -> u32 {
    let mut xmas_list: Vec<String> = vec![];
    let mut sum = 0_u32;

    //NW
    if y >= 3 && x >= 3 {
        xmas_list.push(vec![data[y-3][x-3],data[y-2][x-2],data[y-1][x-1],data[y][x]].iter().collect());
    }
    //N
    if y >= 3 {
        xmas_list.push(vec![data[y-3][x],data[y-2][x],data[y-1][x],data[y][x]].iter().collect());
    }
    //NE
    if y >= 3 && x <= data[y].len()-4 {
        xmas_list.push(vec![data[y-3][x+3],data[y-2][x+2],data[y-1][x+1],data[y][x]].iter().collect());
    }
    //E
    if x <= data[y].len()-4 {
        xmas_list.push(vec![data[y][x+3],data[y][x+2],data[y][x+1],data[y][x]].iter().collect());
    }
    //SE
    if y <= data.len()-4 && x <= data[y].len()-4 {
        xmas_list.push(vec![data[y+3][x+3],data[y+2][x+2],data[y+1][x+1],data[y][x]].iter().collect());
    }
    //S
    if y <= data.len()-4 {
        xmas_list.push(vec![data[y+3][x],data[y+2][x],data[y+1][x],data[y][x]].iter().collect());
    }
    //SW
    if y <= data.len()-4 && x >= 3 {
        xmas_list.push(vec![data[y+3][x-3],data[y+2][x-2],data[y+1][x-1],data[y][x]].iter().collect());
    }
    //W
    if x >= 3 {
        xmas_list.push(vec![data[y][x-3],data[y][x-2],data[y][x-1],data[y][x]].iter().collect());
    }

    // xmas_list.iter().filter(|s| s.eq(&"XMAS") || s.eq(&"SAMX")).collect::<String>().len() as u32

    for str in xmas_list {
        if str.eq(&"XMAS") || str.eq(&"SAMX") { sum += 1; }
    }

    sum
}

fn part2(data: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0_u32;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == 'A' {
                sum += find_mas(y, x, &data);
            }
        }
    }

    sum
}

fn part1(data: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0_u32;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == 'X' {
                sum += find_xmas(y, x, &data);
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(9_u32, part2(&build_input(&"input-sample.txt".to_owned())));
    }

    #[test]
    fn test_part1() {
        assert_eq!(18_u32, part1(&build_input(&"input-sample.txt".to_owned())));
    }
}
