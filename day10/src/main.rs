fn main() {
    let starting_string = "1113122113";
    let mut previous_string = String::from(starting_string);
    for i in 1..=50 {
        let mut res = String::new();
        let mut char_count = 0;
        let mut previous_char = 'A';
        for char in previous_string.chars() {
            if char == previous_char {
                char_count += 1;
            }
            if char != previous_char {
                if char_count != 0 {
                    res += &(char_count.to_string() + &String::from(previous_char));
                }
                previous_char = char;
                char_count = 1;
            }
        }
        res += &(char_count.to_string() + &String::from(previous_char));
        let len = res.len();
        println!("iteration {i}: {len}");
        previous_string = res;
    }
}
