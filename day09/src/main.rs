use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();
    let mut city_index = HashMap::new();
    let mut m = vec![vec![0; 8]; 8];
    let mut next_entry = 0;
    let mut min_cost = -1;
    let mut max_cost = 0;
    let mut best_perm: Vec<usize> = vec![0; 8];
    let mut worst_perm: Vec<usize> = vec![0; 8];
    for line in f.lines() {
        let line_split: Vec<&str> = line.split(' ').collect();
        let city_a = line_split[0];
        let city_b = line_split[2];
        let cost = line_split[4];
        next_entry = city_index.len();
        city_index.entry(city_a).or_insert(next_entry);
        next_entry = city_index.len();
        city_index.entry(city_b).or_insert(next_entry);
        m[*city_index.get(city_a).unwrap()][*city_index.get(city_b).unwrap()] =
            cost.parse().unwrap();
        m[*city_index.get(city_b).unwrap()][*city_index.get(city_a).unwrap()] =
            cost.parse().unwrap();
    }
    let perms = (0..8).permutations(8);
    for perm in perms {
        let c = perm_cost(perm.clone(), m.clone());
        if (min_cost == -1) || (min_cost > c) {
            min_cost = c;
            best_perm = perm.clone();
        }
        if max_cost < c {
            max_cost = c;
            worst_perm = perm.clone();
        }
    }
    let mut cities = vec![""; 8];
    for (city, idx) in city_index {
        cities[idx] = city;
    }
    let result: Vec<&str> = best_perm.iter().map(|&idx| cities[idx]).collect();
    println!("min cost:{min_cost} with perm: {:?}", result);
    let result: Vec<&str> = worst_perm.iter().map(|&idx| cities[idx]).collect();
    println!("max cost:{max_cost} with perm: {:?}", result);
}

fn perm_cost(perm: Vec<usize>, m: Vec<Vec<i32>>) -> i32 {
    let mut c = 0;
    for travel in perm.windows(2) {
        let (a, b) = (travel[0], travel[1]);
        c += m[a][b];
    }
    c
}
