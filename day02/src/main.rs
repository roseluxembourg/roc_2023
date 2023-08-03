use std::fs;
fn main() {
    let f = fs::read_to_string("input.txt").expect("File not found");
    let mut sum = 0;
    let mut ribbon = 0;
    for line in f.lines() {
        let mut splits = line.split("x");
        let (l, h, w) = (
            splits
                .next()
                .expect("line uncomplete")
                .parse::<i32>()
                .unwrap(),
            splits
                .next()
                .expect("line uncomplete")
                .parse::<i32>()
                .unwrap(),
            splits
                .next()
                .expect("line uncomplete")
                .parse::<i32>()
                .unwrap(),
        );
        let (a, b, c) = (l * h, l * w, h * w);
        sum += 2 * (a + b + c) + [a, b, c].iter().min().expect("min");
        ribbon += 2 * (l + w + h - [l, w, h].iter().max().expect("max")) + l * w * h;
    }
    println!("{sum}");
    println!("{ribbon}");
}
