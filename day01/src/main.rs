use std::fs;

fn main() {
    let contents = fs::read_to_string("input_d1.txt").expect("File should be read");
    let mut count = 0;
    let mut encountered_basement = 0;
    for (i, char) in contents.chars().enumerate() {
        if char == '(' {
            count += 1;
        } else {
            count -= 1;
            if (count < 0) && (encountered_basement == 0) {
                encountered_basement = i + 1;
            }
        }
    }
    println!("Final floor: {count}");
    println!("Entering basement at position: {encountered_basement}");
}
