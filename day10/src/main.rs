use itertools::Itertools;

fn main() {
    let input = vec![1, 3, 2, 1, 1, 3, 1, 1, 1, 2];

    let now = std::time::Instant::now();
    println!("part 1: {} ({:?})", part_1(&input, 40), now.elapsed());
    let now = std::time::Instant::now();
    println!("part 2: {} ({:?})", part_1(&input, 50), now.elapsed());
}

fn part_1(input: &Vec<u8>, times: usize) -> usize {
    let mut count = 0;
    let mut start = input.clone();

    while count < times {
        count += 1;
        start = look_and_say(start);
    }

    start.len()
}

fn look_and_say(input: Vec<u8>) -> Vec<u8> {
    input
        .into_iter()
        .chunk_by(|&x| x)
        .into_iter()
        .flat_map(|(_, group)| {
            let group_vec = group.collect::<Vec<_>>();
            [group_vec.len() as u8, group_vec[0]]
        })
        .collect::<Vec<_>>()
}
