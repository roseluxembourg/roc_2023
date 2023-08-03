use std::fs;

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();
    let mut count = 0;
    let mut count_2 = 0;
    for line in f.lines() {
        let len_lit = line.len();
        let mut prev_char = 'A';
        let mut len_mem = len_lit - 2;
        let mut len_plus = len_lit + 2;
        for char in line.chars() {
            if char == '\\' {
                len_plus += 1;
                if prev_char != '\\' {
                    len_mem -= 1;
                } else {
                    prev_char = 'A';
                    continue;
                }
            }
            if char == '\"' {
                len_plus += 1;
            }
            if (char == 'x') & (prev_char == '\\') {
                len_mem -= 2;
            }
            prev_char = char;
        }
        count += len_lit - len_mem;
        count_2 += len_plus - len_lit;
        println!("{line} ; {len_lit} ; {len_plus}");
    }
    println!("{count}");
    println!("{count_2}");
}
