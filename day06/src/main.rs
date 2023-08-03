use regex::Regex;
use std::cmp;
use std::fs;

fn main() {
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let re = Regex::new(r"(?<command>turn on|turn off|toggle) (?<begin>[0-9]+,[0-9]+) through (?<end>[0-9]+,[0-9]+)$").unwrap();
    let f = fs::read_to_string("input.txt").unwrap();
    for line in f.lines() {
        let caps = re.captures(line).unwrap();
        let mut begin_it = caps["begin"].split(",").map(|s| s.parse().unwrap());
        let begin: (usize, usize) = (begin_it.next().unwrap(), begin_it.next().unwrap());

        let mut end_it = caps["end"].split(",").map(|s| s.parse().unwrap());
        let end: (usize, usize) = (end_it.next().unwrap(), end_it.next().unwrap());
        let command = &caps["command"];
        match command {
            "turn on" => turn_on(&mut grid, begin, end),
            "turn off" => turn_off(&mut grid, begin, end),
            "toggle" => toggle(&mut grid, begin, end),
            _ => {}
        }
    }
    println!(
        "{}",
        grid.iter()
            .map(|subvec| subvec.iter().sum::<i32>())
            .sum::<i32>()
    );
}

fn turn_on(grid: &mut Vec<Vec<i32>>, begin: (usize, usize), end: (usize, usize)) {
    for x in begin.0..=end.0 {
        for y in begin.1..=end.1 {
            //grid[x][y] = 1;
            grid[x][y] += 1;
        }
    }
}

fn turn_off(grid: &mut Vec<Vec<i32>>, begin: (usize, usize), end: (usize, usize)) {
    for x in begin.0..=end.0 {
        for y in begin.1..=end.1 {
            grid[x][y] = cmp::max(0, grid[x][y] - 1);
        }
    }
}

fn toggle(grid: &mut Vec<Vec<i32>>, begin: (usize, usize), end: (usize, usize)) {
    for x in begin.0..=end.0 {
        for y in begin.1..=end.1 {
            //grid[x][y] = 1 - grid[x][y];
            grid[x][y] += 2;
        }
    }
}
