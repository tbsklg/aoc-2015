fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();
    println!("part 1: {} ({:?})", part_1(&input), now.elapsed());
}

fn part_1(input: &str) -> usize {
    input.lines().filter_map(is_nice).count()
}

fn is_nice(line: &str) -> Option<&str> {
    let vowel_count = |line: &str| {
        line.chars()
            .filter(|x| ['a', 'e', 'i', 'o', 'u'].contains(x))
            .count()
    };
    let has_double_row = |line: &str| {
        line.chars()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|pair| pair[0] == pair[1])
    };
    let forbidden = |line: &str| ["ab", "cd", "pq", "xy"].iter().any(|x| line.contains(x));

    if vowel_count(line) > 2 && has_double_row(line) && !forbidden(line) {
        Some(line)
    } else {
        None
    }
}
