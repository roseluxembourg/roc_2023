use md5;

fn main() {
    let secret_key = "yzbqklnj";
    let mut index = 1;
    loop {
        let input = secret_key.to_owned() + &index.to_string();
        let digest = md5::compute(input.into_bytes());
        let result = format!("{:x}", digest);
        if &result[..6] == "000000" {
            break;
        }
        index += 1;
    }
    println!("min bitcoin index is {index}")
}
