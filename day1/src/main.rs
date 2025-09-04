fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> i32 {
    input
        .chars()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .chars()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .position(|x| x == -1)
        .unwrap()
        + 1
}
