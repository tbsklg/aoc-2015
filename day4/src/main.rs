#![warn(clippy::pedantic)]

use md5::{Digest, Md5};
use rayon::iter::{IntoParallelIterator as _, ParallelIterator};
use std::env::args;

fn main() {
    let input = args().nth(1).unwrap();

    let now = std::time::Instant::now();
    println!(
        "part 1: {} ({:?})",
        solve(&input, 5).unwrap(),
        now.elapsed()
    );
    println!(
        "part 1: {} ({:?})",
        solve(&input, 6).unwrap(),
        now.elapsed()
    );
}

fn solve(input: &str, leading_zeros: usize) -> Option<usize> {
    let target = "0".repeat(leading_zeros);

    let chunk_size = 100_000;

    (1..).step_by(chunk_size).find_map(|start| {
        (start..start + chunk_size)
            .into_par_iter()
            .find_first(|&i| {
                let mut hasher = Md5::new();
                hasher.update(input.as_bytes());
                hasher.update(i.to_string().as_bytes());
                let result = hasher.finalize();
                format!("{result:x}").starts_with(&target)
            })
    })
}
