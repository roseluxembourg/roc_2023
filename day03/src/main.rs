use std::collections::HashMap;
use std::fs;

fn main() {
    let f = fs::read_to_string("input.txt").expect("File not found");
    //let mut pos: (i32, i32) = (0, 0);
    let mut pos1: (i32, i32) = (0, 0);
    let mut pos2: (i32, i32) = (0, 0);
    let mut explored = HashMap::new();
    explored.insert((0, 0), 1);
    let mut count = 0;
    for (i, char) in f.chars().enumerate() {
        //pos = move_space(&mut pos, char);
        //explored.entry(pos).or_insert(1);
        if i % 2 == 0 {
            pos1 = move_space(&mut pos1, char);
            explored.entry(pos1).or_insert(1);
        } else {
            pos2 = move_space(&mut pos2, char);
            explored.entry(pos2).or_insert(1);
        }
    }
    for (_key, _value) in &explored {
        count += 1;
    }
    println!("{count}");
}

fn move_space(position: &mut (i32, i32), character: char) -> (i32, i32) {
    match character {
        '^' => *position = (position.0, position.1 + 1),
        'v' => *position = (position.0, position.1 - 1),
        '>' => *position = (position.0 + 1, position.1),
        '<' => *position = (position.0 - 1, position.1),
        _ => println!("Failed!"),
    }
    *position
}
