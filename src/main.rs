use std::collections::HashMap;
use std::io::{stdin, Read};

mod questions;

fn parse_vec_int_from_str(first_line: Option<&str>) -> Vec<u32> {
    return first_line
        .unwrap_or("")
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<u32>().ok().expect("Parse error."))
        .collect();
}
fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("Parse error.");

    let mut str_iterator = buf.lines();
    //let _zero = str_iterator.next(); // skip first line
    let array_from_str = parse_vec_int_from_str(str_iterator.next());

    let mut hashmap = HashMap::new();
    for color in array_from_str {
        *hashmap.entry(color).or_insert(0) += 1;
    }
    let mut count = 0;
    for (&_key, &val) in hashmap.iter() {
        if val < 2 {
            continue;
        }
        count += val / 2;
    }
    println!("{}", count);
}
