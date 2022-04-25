use std::io::{stdin, Read};

pub mod questions;

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).expect("Error.");
    for ch in buf.chars() {
        println!("{}", ch);
    }
}
