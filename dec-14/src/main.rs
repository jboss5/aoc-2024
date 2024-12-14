use aoc_utils::get_lines_str;
use regex::Regex;

#[derive(Debug)]
struct RobotPath {
    p_x: i32,
    p_y: i32,
    v_x : i32,
    v_y : i32
}

//- 2,4 => 2,-3
//1 4,1
//2 6,5
//3 2,8
//4 6,10
//5 3,1
fn print_robots(robots: &Vec<RobotPath>, y: usize, x: usize) {
    let mut grid = vec![vec!['.'; x]; y];
    robots.iter().for_each(|r| {
       grid[r.p_y as usize][r.p_x as usize] = '#';
    });

    for h in 0..grid.len() {
        for w in 0..grid[h].len() {
            print!("{}", grid[h][w]);
        }
        println!();
    }
    // println!("{:?}", grid);
}

fn main() {
    let mut data = build_input("input.txt");
    // println!("data: {:?}", data);

    // let mut robot = data.get_mut(10).unwrap();
    let wid = 100;
    let h = 102;

    // let wid = 10;
    // let h = 6;

    for it in 0..15000 {
        println!("ITERATION: {}", it);
        for mut robot in data.iter_mut() {
            // println!("{:?}", robot);
            let mut new_x = robot.p_x + robot.v_x;
            let mut new_y = robot.p_y + robot.v_y;

            // println!("x {}, y {}", new_x, new_y);

            if new_x > wid { new_x = (wid + 1 - new_x).abs(); }
            // if new_y > h { new_y = h - new_y; }
            if new_y > h { new_y = (h + 1 - new_y).abs(); }
            if new_x < 0 { new_x = wid + 1 + new_x; }
            if new_y < 0 { new_y = h + 1 + new_y; }

            robot.p_x = new_x;
            robot.p_y = new_y;
        }

        if it > 500 {
            print_robots(&data, (h + 1) as usize, (wid + 1) as usize);
            println!();
        }
    }

    // println!("data: {:?}", data);

    let x_mid = wid / 2;
    let y_mid = h / 2;
    let (mut q1_cnt, mut q2_cnt, mut q3_cnt, mut q4_cnt) = (0, 0, 0, 0);
    for robot in data {
        if robot.p_x < x_mid && robot.p_y < y_mid { q1_cnt += 1; }
        if robot.p_x > x_mid && robot.p_y < y_mid { q2_cnt += 1; }
        if robot.p_x < x_mid && robot.p_y > y_mid { q3_cnt += 1; }
        if robot.p_x > x_mid && robot.p_y > y_mid { q4_cnt += 1; }
    }
    let p1 = q1_cnt * q2_cnt * q3_cnt * q4_cnt;
    // println!("cnts {} {} {} {}", q1_cnt, q2_cnt, q3_cnt, q4_cnt);
    println!("p1: {}", p1);
}


fn build_input(file: &str) -> Vec<RobotPath> {
    let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").expect("Error building regex");
    let mut robot_vec = vec![];
    get_lines_str(file).for_each(|l| {
        regex.captures_iter(l.unwrap().as_str()).for_each(|c| {
            robot_vec.push(RobotPath {
                p_x: c[1].parse::<i32>().expect("Invalid c1"),
                p_y: c[2].parse::<i32>().expect("Invalid c2"),
                v_x: c[3].parse::<i32>().expect("Invalid c3"),
                v_y: c[4].parse::<i32>().expect("Invalid c4"),
            });
        });
    });

    robot_vec
}