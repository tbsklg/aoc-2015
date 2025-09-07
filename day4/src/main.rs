use md5::{Digest, Md5};
use std::env::args;

fn main() {
    let input = args().nth(1).unwrap();

    println!("part 1: {}", solve(&input, 5).unwrap());
    println!("part 1: {}", solve(&input, 6).unwrap());
}

fn solve(input: &str, leading_zeros: usize) -> Option<usize> {
    for i in 1.. {
        let mut hasher = Md5::new();
        hasher.reset();
        hasher.update(input.as_bytes());
        hasher.update(i.to_string().as_bytes());
        let result = hasher.finalize();

        if format!("{:x}", result).starts_with(&"0".repeat(leading_zeros)) {
            return Some(i);
        }
    }

    None
}
