use itertools::Itertools;

fn main() {
    part_1(vec![1, 3, 2, 1, 1, 3, 1, 1, 1, 2]);
}

fn part_1(input: Vec<u8>) -> usize {
    let g = input
        .into_iter()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    println!("{:?}", g);

    todo!()
}
