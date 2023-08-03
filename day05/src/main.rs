use regex::Regex;
use std::fs;

fn main() {
    let f = fs::read_to_string("input.txt").expect("File not found");
    let cond1 = Regex::new(r"(.*[aeiou].*){3}").unwrap();
    let cond3 = Regex::new(r"(ab)|(cd)|(pq)|(xy)").unwrap();
    let mut count = 0;
    for line in f.lines() {
        let res1 = cond1.find(line);
        match res1 {
            Some(_) => {}
            None => continue,
        }
        let mut prev_char = 'A';
        let mut found = false;
        for char in line.chars() {
            if char == prev_char {
                found = true;
                continue;
            }
            prev_char = char;
        }
        if found == false {
            continue;
        }
        let res3 = cond3.find(line);
        match res3 {
            Some(_) => continue,
            None => {}
        }
        count += 1;
        println!("{count}: {line}");
    }
    println!("{count}")
}
